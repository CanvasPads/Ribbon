/// A location information for AST nodes.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ASTLoc {
    pub start: u32,
    pub end: u32,
}

/// A node that has [`ASTLoc`] in own member.
pub(crate) trait ASTHasLoc {
    fn loc(&self) -> ASTLoc;
}

pub struct ASTNodeViewElement {
    loc: ASTLoc,
}

impl ASTHasLoc for ASTNodeViewElement {
    fn loc(&self) -> ASTLoc {
        self.loc
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ASTItemConst {}

/// AST nodes that possibly placement in a block
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ASTNodeScoped {
    Const(ASTItemConst),
}

/// A module node
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ASTNodeModule {
    loc: ASTLoc,
    pub name: String,
    pub nodes: Vec<ASTNodeScoped>,
}

impl ASTHasLoc for ASTNodeModule {
    fn loc(&self) -> ASTLoc {
        self.loc
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TokenLoc {
    pub starts_at: u32,
    pub len: u32,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum TokenLiteral {
    NumberLiteral(String),
    StringLiteral(String),
}

impl TokenLiteral {
    pub(crate) fn content(&self) -> &String {
        match self {
            TokenLiteral::NumberLiteral(s) => s,
            TokenLiteral::StringLiteral(s) => s,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum TokenContent {
    Anchor(String),
    Identifier(String),
    Literal(TokenLiteral),
    VTagStartPre { name: String },  // <tag-name
    VTagStartPost { name: String }, // </tag-name
    VTagSelfClosing,                // />
    VTagAttrName(String),           // [[name]]: Type = "value"
    VTagEnd,                        // >
    BraceLeft,
    BraceRight,
    AddOp,
    AssignmentOp,
    BitwiseAndOp,
    Create,
    Const,
    Delete,
    Else,
    Enum,
    FromKeyword,
    FnKeyword,
    For,
    If,
    Int,
    Insert,
    Key,
    Let,
    New,
    Nil,
    Number,
    StringKeyword,
    Tag,
    Type,
    View,
    With,
    Or,
    Priv,
    Replace,
}

impl TokenContent {
    pub fn from_str(word: &str) -> Option<Self> {
        match word.to_lowercase().as_str() {
            "create" => Some(Self::Create),
            "const" => Some(Self::Const),
            "delete" => Some(Self::Delete),
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "from" => Some(Self::FromKeyword),
            "for" => Some(Self::For),
            "if" => Some(Self::If),
            "int" => Some(Self::Int),
            "insert" => Some(Self::Insert),
            "key" => Some(Self::Key),
            "let" => Some(Self::Let),
            "new" => Some(Self::New),
            "number" => Some(Self::Number),
            "string" => Some(Self::StringKeyword),
            "tag" => Some(Self::Tag),
            "type" => Some(Self::Type),
            "with" => Some(Self::With),
            "or" => Some(Self::Or),
            "priv" => Some(Self::Priv),
            "replace" => Some(Self::Replace),
            _ => None,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '{' => Some(Self::BraceLeft),
            '}' => Some(Self::BraceRight),
            '=' => Some(Self::AssignmentOp),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: TokenLoc,
    pub con: TokenContent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeLoc {
    start: u32,
    end: u32,
}

pub trait Node {
    fn loc(&mut self) -> NodeLoc;
    fn set_loc(&mut self, loc: NodeLoc);
}

pub trait HasChildren {
    fn append_child<T: Node>(&mut self, node: T);
    fn children(&mut self) -> Vec<Box<dyn Node>>;
}

pub trait Query: Node {}

pub trait Scope: Node + HasChildren {}

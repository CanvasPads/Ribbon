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
pub struct ASTItemConst {
    loc: ASTLoc,
}

impl ASTHasLoc for ASTItemConst {
    fn loc(&self) -> ASTLoc {
        self.loc
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ASTItemView {
    loc: ASTLoc,
}

impl ASTHasLoc for ASTItemView {
    fn loc(&self) -> ASTLoc {
        self.loc
    }
}

/// AST nodes that possibly placement in a block
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ASTNodeScoped {
    Const(ASTItemConst),
    View(ASTItemView),
}

impl ASTHasLoc for ASTNodeScoped {
    fn loc(&self) -> ASTLoc {
        match self {
            ASTNodeScoped::Const(i) => i.loc(),
            ASTNodeScoped::View(i) => i.loc(),
        }
    }
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
    /// `#anchor`
    Anchor(String),
    /// `variable_name, function_name, CONSTANT_VALUE, ObjectName`
    Identifier(String),
    /// `"hello, world", 1, 0xdeadbeef`
    Literal(TokenLiteral),
    /// `(`
    ParenthesisLeft,
    /// `)`
    ParenthesisRight,
    /// `{`
    BraceLeft,
    /// `{`
    BraceRight,
    /// `[`
    SquareBracketLeft,
    /// `]`
    SquareBracketRight,
    /// `<`
    TagAngleBracketLeft,
    /// `</`
    TagAngleClosingLeft,
    /// `/>`
    TagAngleSelfClosingRight,
    /// `>`
    TagAngleBracketRight,
    /// `+`
    AddOp,
    /// `=`
    AssignmentOp,
    /// `&`
    BitwiseAndOp,
    /// as
    As,
    /// `const`
    Const,
    /// `effect`
    Effect,
    /// `else`
    Else,
    /// `emits`
    Emits,
    /// `fn`
    FnKeyword,
    /// `for`
    For,
    /// `from`
    FromKeyword,
    /// `if`
    If,
    /// `import`
    Import,
    /// `let`
    Let,
    /// `nil`
    Nil,
    /// `type`
    Type,
    /// `use`
    Use,
    /// `view`
    View,
    /// `when`
    When,
    /// `with`
    With,
    /// `pub`
    Pub,
}

impl TryFrom<&str> for TokenContent {
    type Error = ();
    fn try_from(word: &str) -> Result<Self, Self::Error> {
        match word {
            "</" => Ok(Self::TagAngleClosingLeft),
            "/>" => Ok(Self::TagAngleSelfClosingRight),
            "as" => Ok(Self::As),
            "const" => Ok(Self::Const),
            "effect" => Ok(Self::Effect),
            "else" => Ok(Self::Else),
            "emits" => Ok(Self::Emits),
            "fn" => Ok(Self::FnKeyword),
            "for" => Ok(Self::For),
            "from" => Ok(Self::FromKeyword),
            "if" => Ok(Self::If),
            "import" => Ok(Self::Import),
            "let" => Ok(Self::Let),
            "nil" => Ok(Self::Nil),
            "type" => Ok(Self::Type),
            "use" => Ok(Self::Use),
            "view" => Ok(Self::View),
            "when" => Ok(Self::When),
            "with" => Ok(Self::With),
            "pub" => Ok(Self::Pub),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for TokenContent {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Self::ParenthesisLeft),
            ')' => Ok(Self::ParenthesisRight),
            '{' => Ok(Self::BraceLeft),
            '}' => Ok(Self::BraceRight),
            '=' => Ok(Self::AssignmentOp),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: TokenLoc,
    pub con: TokenContent,
}

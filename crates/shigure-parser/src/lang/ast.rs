pub mod item;

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
    /// `a_variable`, `a_function`, `CONSTANT_VALUE`, `TypeName`
    Identifier(String),
    /// `"hello, world"`, `2`, `0xdeadbeef`
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
    /// `fn`
    FnKeyword,
    /// `for`
    For,
    /// `from`
    FromKeyword,
    /// `if`
    If,
    /// `impl`
    Impl,
    /// `import`
    Import,
    /// `in`
    In,
    /// `var`
    Var,
    /// `let`
    Let,
    /// `proto`
    Proto,
    /// `type`
    Type,
    /// `undefined`
    Undefined,
    /// `use`
    Use,
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
            "fn" => Ok(Self::FnKeyword),
            "for" => Ok(Self::For),
            "from" => Ok(Self::FromKeyword),
            "if" => Ok(Self::If),
            "impl" => Ok(Self::Impl),
            "import" => Ok(Self::Import),
            "var" => Ok(Self::Var),
            "let" => Ok(Self::Let),
            "proto" => Ok(Self::Proto),
            "undefined" => Ok(Self::Undefined),
            "type" => Ok(Self::Type),
            "use" => Ok(Self::Use),
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

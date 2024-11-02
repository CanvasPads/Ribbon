use crate::lang::ast::Token;
use crate::lang::tokenizer::TokenizerErr;

pub mod module;
pub mod view;

pub trait Parser<T> {
    fn parse_all(&self) -> ParseResult<T>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseError {
    /// The token type is unexpected.
    UnexpectedToken,
    /// Incorrect syntax
    SyntaxError,
    TokenizeError(TokenizerErr),
}

pub type ParseResult<T> = Result<T, ParseError>;
pub type TokenizeResult = Result<Token, ParseError>;

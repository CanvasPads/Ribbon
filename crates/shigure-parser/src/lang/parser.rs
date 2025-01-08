use std::fmt::Debug;

use shigure_log::{Logger, Message, MessageLevel};

use crate::lang::{
    ast::{
        item::{
            Loc, NodeAssignmentOp, NodeFile, NodeIdentifier, NodeModule, NodeNamespace,
            NodeParameter, NodeScoped, NodeStringLiteral, NodeStructured, NodeValue,
        },
        Token, TokenContent,
    },
    tokenizer::{TokenResult, Tokenizer, TokenizerErr},
};

pub struct Parser<'a> {
    filename: &'a String,
    tokenizer: Tokenizer<'a>,
    logger: Logger<'a>,
    previous: Option<TokenResult>,
    current: Option<TokenResult>,
}

#[derive(Debug)]
pub struct ParseMessageHint {}

#[derive(Debug)]
pub enum ParseMessageKind {
    Error,
    Warning,
    Info,
}

pub struct ParseMessage {
    kind: ParseMessageKind,
    hints: Vec<ParseMessageHint>,
}

#[derive(Debug)]
pub enum ParseError {
    TokenizeError { loc: Loc, error: TokenizerErr },
    SyntaxError { loc: Loc },
}

impl ParseError {
    pub fn loc(&self) -> Loc {
        match self {
            Self::SyntaxError { loc, .. } => loc.clone(),
            Self::TokenizeError { loc, .. } => loc.clone(),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

impl<'a> Parser<'a> {
    pub fn new(filename: &'a String, input: &'a String) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        let logger = Logger::new(filename, input);
        let current = tokenizer.next();
        Parser {
            filename,
            tokenizer,
            logger,
            previous: None,
            current,
        }
    }

    fn syntax_error(&self, title: &str, loc: Loc) -> ParseError {
        let message = Message {
            level: MessageLevel::Error,
            pos: loc.start,
            title: title.into(),
            hints: vec![],
        };
        self.logger.issue(message);
        ParseError::SyntaxError { loc }
    }

    fn tokenize_error(&self, error: TokenizerErr, loc: Loc) -> ParseError {
        let title = match error.clone() {
            TokenizerErr::UnexpectedToken { loc } => "Unexpected token",
            TokenizerErr::UnterminatedStringLiteral { loc } => "Unterminated string",
            _ => "Tokenizer error",
        }
        .to_string();
        let message = Message {
            level: MessageLevel::Error,
            pos: loc.start,
            title,
            hints: vec![],
        };
        self.logger.issue(message);
        ParseError::TokenizeError { loc, error }
    }

    fn try_parsing_identifier(&mut self) -> ParseResult<Option<NodeIdentifier>> {
        let tok = self.unwrap_current()?;
        if let TokenContent::Identifier(idef) = tok.con {
            Ok(Some(NodeIdentifier {
                value: idef,
                loc: tok.loc.into(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Read the current token and parse it as an identifier.
    fn expect_identifier(&mut self) -> ParseResult<NodeIdentifier> {
        let tok = self.unwrap_current()?;
        if let Some(ident) = self.try_parsing_identifier()? {
            Ok(ident)
        } else {
            Err(self.syntax_error("Invalid identifier", tok.loc.into()))
        }
    }

    fn expect_assignment_op(&mut self) -> ParseResult<NodeAssignmentOp> {
        let tok = self.unwrap_current()?;
        if let TokenContent::AssignmentOp = tok.con {
            Ok(NodeAssignmentOp {
                loc: tok.loc.into(),
            })
        } else {
            Err(ParseError::SyntaxError {
                loc: tok.loc.into(),
            })
        }
    }

    fn expect_structured(&mut self) -> ParseResult<NodeStructured> {
        let tok = self.unwrap_current()?;
        match tok.con {
            TokenContent::BraceLeft => loop {
                self.consume_token();
            },
            _ => Err(self.syntax_error("Invalid structured", tok.loc.into())),
        }
    }

    /// Parse value such as structs, variables and more
    fn expect_value(&mut self) -> ParseResult<NodeValue> {
        let tok = self.unwrap_current()?;
        match tok.con {
            TokenContent::BraceLeft => {
                let structured = self.expect_structured()?;
                Ok(NodeValue::Structured(structured))
            }
            TokenContent::Identifier(..) => {
                let ident = self.expect_identifier()?;
                Ok(NodeValue::Identifier(ident))
            }

            _ => Err(ParseError::SyntaxError),
        }
    }

    fn parse_module(&mut self) -> ParseResult<NodeModule> {
        let start = self.get_tokenizer_idx();
        let nodes: Vec<NodeScoped> = Vec::new();
        while let Some(res) = self.unwrap_current_or_none()? {
            match res.con {
                TokenContent::Let => {
                    // let
                    self.consume_token();
                    // <name>
                    let name = self.expect_identifier()?;
                    self.consume_token();
                    // function parameters
                    if let Ok(..) = self.expect_params() {
                        self.consume_token();
                    }
                    self.expect_assignment_op()?;
                    self.consume_token();
                    // <value>
                    let value = self.expect_value()?;
                    self.consume_token();
                }
                TokenContent::Import => {
                    // import
                    self.consume_token();
                    if let Some(tok) = self.try_parsing_string_literal()? {
                        // "<url>"
                    } else if let Some(tok) = self.try_parsing_namespace()? {
                        // <item>
                    } else {
                        return Err(self.syntax_error("Unexpected value", res.loc.into()));
                    }
                }
                TokenContent::Identifier(ident) => {
                    // <identifier>
                    self.consume_token();
                    let ident = self.expect_identifier()?;
                }
                _ => {
                    self.consume_token();
                }
            }
        }
        Ok(NodeModule {
            loc: Loc {
                start,
                end: self.get_tokenizer_idx(),
            },
            name,
            nodes,
        })
    }

    fn unwrap_current(&mut self) -> Result<Token, ParseError> {
        match self.current.clone() {
            Some(tok_res) => match tok_res {
                Ok(tok) => Ok(tok),
                Err(err) => Err(self.tokenize_error(err, err.loc().into())),
            },
            None => {
                let loc = self.tokenizer.get_current_loc();
                Err(self.syntax_error("Unterminated token", loc.into()))
            }
        }
    }

    fn unwrap_current_or_none(&mut self) -> Result<Option<Token>, ParseError> {
        if let Some(tok_res) = self.current.clone() {
            match tok_res {
                Ok(tok) => Ok(Some(tok)),
                Err(err) => Err(self.tokenize_error(err, err.loc().into())),
            }
        } else {
            Ok(None)
        }
    }

    fn consume_token(&mut self) {
        let next = self.tokenizer.next();
        let prev = self.current.clone();
        self.previous = prev;
        self.current = next;
    }

    fn get_tokenizer_idx(&self) -> usize {
        self.tokenizer.get_current_idx()
    }

    fn parse_file(&mut self) -> ParseResult<NodeFile> {
        let start = self.get_tokenizer_idx();
        let module = self.parse_module("<file>".into())?;
        let end = self.get_tokenizer_idx();
        Ok(NodeFile {
            loc: Loc { start, end },
            name: self.filename.clone(),
            modules: vec![module],
        })
    }

    pub fn parse_all(&mut self) -> ParseResult<NodeFile> {
        self.parse_file()
    }
}

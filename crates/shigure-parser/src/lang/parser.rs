use crate::lang::{
    ast::{
        item::{Loc, NodeAssignmentOp, NodeFile, NodeIdentifier, NodeModule},
        Token, TokenContent, TokenLoc,
    },
    tokenizer::{TokenResult, Tokenizer, TokenizerErr},
};

pub struct Parser<'a> {
    filename: String,
    tokenizer: Tokenizer<'a>,
    current: Option<TokenResult>,
}

pub enum ParseError {
    TokenizeError(TokenizerErr),
}

pub type ParseResult<T> = Result<T, ParseError>;

impl<'a> Parser<'a> {
    pub fn new(filename: String, input: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        let current = tokenizer.next();
        Parser {
            filename,
            tokenizer,
            current,
        }
    }

    /// Read the current token and parse it as an identifier.
    fn expect_identifier(&mut self) -> ParseResult<NodeIdentifier> {
        let tok = self.unwrap_current()?;
        if let TokenContent::Identifier(idef) = tok.con {
            Ok(NodeIdentifier {
                value: idef,
                loc: tok.loc.into(),
            })
        } else {
            Err(ParseError::SyntaxError)
        }
    }

    fn expect_assignment_op(&mut self) -> ParseResult<NodeAssignmentOp> {
        let tok = self.unwrap_current()?;
        if let TokenContent::AssignmentOp = tok.con {
            Ok(NodeAssignmentOp {
                loc: tok.loc.into(),
            })
        } else {
            Err(ParseError::SyntaxError)
        }
    }

    fn expect_structured(&mut self) -> ParseResult<NodeStructured> {
        match self.unwrap_current()?.con {
            TokenContent::BraceLeft => loop {
                self.consume_token();
                let tok = self.unwrap_current()?;
            },
            _ => Err(ParseError::SyntaxError),
        }
    }

    /// Parse value such as structs, variables and more
    fn expect_value(&mut self) -> ParseResult<NodeValue> {
        match self.unwrap_current()?.con {
            TokenContent::BraceLeft => {
                let structured = self.expect_structred()?;
                Ok(NodeValue::Structured(structured))
            }
            TokenContent::Identifier => {
                let ident = self.expect_identifier()?;
                Ok(NodeValue::Identifier(ident))
            }

            _ => Err(ParseError::SyntaxError),
        }
    }

    fn parse_module(&mut self) -> ParseResult<NodeModule> {
        let start = self.get_tokenizer_idx();
        while let Some(res) = self.unwrap_current_or_none()? {
            match res.con {
                TokenContent::Let => {
                    // let
                    self.consume_token();
                    // <name>
                    let name = self.expect_identifier()?;
                    self.consume_token();
                    // =
                    self.expect_assignment_op()?;
                    self.consume_token();
                    // <value>
                    let value = self.expect_value()?;
                    self.consume_token();
                }
                TokenContent::Import => {
                    // import
                    self.consume_token();
                    // "<url>"
                    if let Ok(tok) = self.expect_string_litral() {
                        self.consume_token();
                    // <item>
                    } else if let Ok(tok) = self.expect_identifier() {
                        self.consume_token();
                    }
                }
                TokenContent::Identifier(ident) => {
                    // <identifier>
                    self.consume_token();
                }
                _ => {}
            }
        }
    }

    fn unwrap_current(&mut self) -> Result<Token, ParseError> {
        if let Some(tok_res) = self.current.clone() {
            match tok_res {
                Ok(tok) => Ok(tok),
                Err(err) => Err(ParseError::TokenizeError(err)),
            }
        } else {
            Err(ParseError::SyntaxError)
        }
    }

    fn unwrap_current_or_none(&mut self) -> Result<Option<Token>, ParseError> {
        if let Some(tok_res) = self.current.clone() {
            match tok_res {
                Ok(tok) => Ok(Some(tok)),
                Err(err) => Err(ParseError::TokenizeError(err)),
            }
        } else {
            Ok(None)
        }
    }

    fn consume_token(&mut self) {
        let next = self.tokenizer.next();
        self.current = next;
    }

    fn get_tokenizer_idx(&self) -> u32 {
        self.tokenizer.get_current_idx()
    }

    fn parse_file(&mut self) -> ParseResult<NodeFile> {
        let start = self.get_tokenizer_idx();
        let module = self.parse_module()?;
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

#[cfg(test)]
mod test;

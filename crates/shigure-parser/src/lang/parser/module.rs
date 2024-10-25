use crate::lang::ast::{ASTLoc, ASTNodeModule, ASTNodeScoped, ASTNodeViewElement, TokenContent};
use crate::lang::parser::{view::ViewParser, ParseError, ParseResult, Parser, TokenizeResult};
use crate::lang::tokenizer::{TokenResult, Tokenizer};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum ModuleParserState {
    PendingToken(TokenResult),
    PendingParseError(ParseError),
    EOF,
    #[default]
    Ready,
}

impl ModuleParserState {
    pub fn is_ready(&self) -> bool {
        match self {
            ModuleParserState::Ready => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn has_pending_token(&self) -> Option<&TokenResult> {
        match self {
            ModuleParserState::PendingToken(token) => Some(token),
            _ => None,
        }
    }
}

pub struct ModuleParser<'a> {
    tokenizer: Rc<RefCell<Tokenizer<'a>>>,
    state: RefCell<ModuleParserState>,
    pending: RefCell<Option<ASTNodeModule>>,
}

pub enum ModuleParserResult {
    Continue,
    ParseError(ParseError),
    Done(ASTNodeModule),
}

impl<'a> Parser<ASTNodeModule> for ModuleParser<'a> {
    fn parse_all(&self) -> ParseResult<ASTNodeModule> {
        loop {
            match self.advance() {
                ModuleParserResult::Done(ast) => return Ok(ast),
                ModuleParserResult::ParseError(err) => return Err(err),
                _ => {}
            }
        }
    }
}

impl<'a> ModuleParser<'a> {
    pub fn new(tokenizer: Rc<RefCell<Tokenizer<'a>>>) -> Self {
        ModuleParser {
            tokenizer,
            state: RefCell::new(ModuleParserState::default()),
            pending: None.into(),
        }
    }

    pub fn from_str(input: &'a str) -> Self {
        Self::new(RefCell::new(Tokenizer::new(input)).into())
    }

    fn set_pending_err(&self, err: ParseError) {
        assert!(self
            .state
            .replace(ModuleParserState::PendingParseError(err))
            .is_ready());
    }

    fn set_state_from_parse_result<T>(&self, res: ParseResult<T>) {
        if let Err(err) = res {
            self.set_pending_err(err);
        }
    }

    /// Consume token and handle tokenize error and returns it as [`ParseError`].
    /// If the inner tokenizer has no consumable token, it returns [`ParseError::SyntaxError`].
    fn consume_token_or_err(&self) -> TokenizeResult {
        match self.consume_token() {
            Some(res) => match res {
                Ok(tok) => Ok(tok),
                Err(err) => Err(ParseError::TokenizeError(err)),
            },
            None => Err(ParseError::SyntaxError),
        }
    }

    fn parse_view_elements(&self) -> ParseResult<ASTNodeViewElement> {
        ViewParser::new(self.tokenizer.clone()).parse_all()
    }

    fn parse_view(&self) -> ParseResult<ASTNodeScoped> {
        todo!()
    }

    fn parse_token(&self, res: TokenResult) {
        /*match res {
            Ok(token) => match token.con {
                TokenContent::Create => {
                    let res = self.parse_create();
                    self.set_state_from_parse_result(res);
                }
                _ => self.set_pending_err(ParseError::UnexpectedToken),
            },
            Err(err) => self.set_pending_err(ParseError::TokenizeError(err)),
        }*/
        todo!()
    }

    fn advance(&self) -> ModuleParserResult {
        match self.state.take() {
            ModuleParserState::Ready => {
                match self.consume_token() {
                    Some(res) => assert!(self
                        .state
                        .replace(ModuleParserState::PendingToken(res))
                        .is_ready()),
                    None => assert!(self.state.replace(ModuleParserState::EOF).is_ready()),
                }
                ModuleParserResult::Continue
            }
            ModuleParserState::PendingToken(token) => {
                self.parse_token(token);
                ModuleParserResult::Continue
            }
            ModuleParserState::PendingParseError(err) => ModuleParserResult::ParseError(err),
            ModuleParserState::EOF => {
                ModuleParserResult::Done(self.pending.take().expect("No pending result"))
            }
        }
    }

    fn consume_token(&self) -> Option<TokenResult> {
        self.tokenizer.borrow_mut().next()
    }
}

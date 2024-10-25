use super::Parser;
use crate::lang::{
    ast::{ASTNodeViewElement, TokenContent},
    parser::{ParseError, ParseResult},
    tokenizer::{TokenResult, Tokenizer},
};
use std::{cell::RefCell, rc::Rc};

pub enum ViewParserResult {
    Continue,
    ParseError(ParseError),
    Done,
}

#[derive(Default)]
enum ViewParserState {
    #[default]
    Ready,
    PendingToken(TokenResult),
    PendingParseError(ParseError),
    EOF,
}

impl ViewParserState {
    fn is_ready(&self) -> bool {
        match self {
            ViewParserState::Ready => true,
            _ => false,
        }
    }
}

pub struct ViewParser<'a> {
    tokenizer: Rc<RefCell<Tokenizer<'a>>>,
    state: RefCell<ViewParserState>,
    pending: RefCell<Option<ASTNodeViewElement>>,
}

impl<'a> Parser<ASTNodeViewElement> for ViewParser<'a> {
    fn parse_all(&self) -> ParseResult<ASTNodeViewElement> {
        loop {
            match self.advance() {
                ViewParserResult::ParseError(err) => return Err(err),
                ViewParserResult::Done => {
                    return Ok(self.pending.take().expect("No pending result"));
                }
                _ => {}
            }
        }
    }
}

impl<'a> ViewParser<'a> {
    pub fn new(tokenizer: Rc<RefCell<Tokenizer<'a>>>) -> Self {
        ViewParser {
            tokenizer,
            state: RefCell::new(ViewParserState::default()),
            pending: None.into(),
        }
    }

    fn parse_xml_tag(&self) -> ParseResult<ASTNodeViewElement> {
        todo!()
    }

    fn set_pending_err(&self, err: ParseError) {
        assert!(self
            .state
            .replace(ViewParserState::PendingParseError(err))
            .is_ready());
    }

    fn set_state_from_parse_result<T>(&self, res: ParseResult<T>) {
        if let Err(err) = res {
            self.set_pending_err(err);
        }
    }

    fn parse_token(&self, res: TokenResult) {
        todo!()
    }

    fn advance(&self) -> ViewParserResult {
        type State = ViewParserState;

        match self.state.take() {
            State::Ready => {
                match self.consume_token() {
                    Some(tok) => assert!(self.state.replace(State::PendingToken(tok)).is_ready()),
                    None => assert!(self.state.replace(State::EOF).is_ready()),
                };

                ViewParserResult::Continue
            }
            State::PendingToken(tok) => {
                self.parse_token(tok);
                ViewParserResult::Continue
            }
            State::PendingParseError(err) => ViewParserResult::ParseError(err),
            State::EOF => ViewParserResult::Done,
        }
    }

    fn consume_token(&self) -> Option<TokenResult> {
        self.tokenizer.borrow_mut().next()
    }
}

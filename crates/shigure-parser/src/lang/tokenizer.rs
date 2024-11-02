use crate::lang::ast::*;
use std::{cell::RefCell, iter::Peekable, str::Chars};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenizerErr {
    UnterminatedStringLiteral,
    UnexpectedToken,
    EmptyElementIdentifier,
    InvalidElementIdentifier,
}

pub type TokenResult = Result<Token, TokenizerErr>;
pub type TokenizationResult = Result<(), TokenizerErr>;

pub struct Tokenizer<'a> {
    itr: Peekable<Chars<'a>>,
    pending: RefCell<Option<Token>>,
    current_idx: u32,
    full_idx_count: u32,
    current: Option<char>,
}

const MAX_IDX_VALUE: u32 = u32::MAX;

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut itr = input.chars().peekable();
        if let Some(char0) = itr.next() {
            Self {
                itr,
                pending: RefCell::new(None),
                current_idx: 0,
                full_idx_count: 0,
                current: Some(char0),
            }
        } else {
            panic!("tokenizer may got zero size string")
        }
    }

    fn lex_number_literal(&mut self) -> TokenResult {
        let mut loc = TokenLoc {
            starts_at: self.current_idx,
            len: 0,
        };

        let mut literal = String::new();
        let mut len = 0;
        while let Some(c) = self.current {
            if c.is_digit(10) {
                literal.push(c);
                len += 1;
                self.consume_char();
            } else {
                break;
            }
        }

        loc.len = len;
        Ok(Token {
            loc,
            con: TokenContent::Literal(TokenLiteral::NumberLiteral(literal)),
        })
    }

    fn lex_string_literal(&mut self) -> TokenResult {
        let mut literal = String::from("\"");
        let mut loc = TokenLoc {
            starts_at: self.current_idx,
            len: 0,
        };

        if self.current != Some('"') {
            return Err(TokenizerErr::UnexpectedToken);
        }

        self.consume_char();

        while let Some(c) = self.current {
            self.consume_char();
            literal.push(c);

            if c == '"' {
                loc.len = self.current_idx - loc.starts_at;
                return Ok(Token {
                    loc,
                    con: TokenContent::Literal(TokenLiteral::StringLiteral(literal)),
                });
            }
        }

        Err(TokenizerErr::UnterminatedStringLiteral)
    }

    fn lex_reserved(&mut self) -> Option<TokenResult> {
        let mut word = String::new();
        let mut loc = TokenLoc {
            starts_at: self.current_idx,
            len: 0,
        };
        while let Some(c) = self.current {
            if c.is_alphabetic() {
                word.push(c);
                self.consume_char();
                if let Ok(con) = TokenContent::try_from(word.as_str()) {
                    loc.len = self.current_idx - loc.starts_at + 1;
                    return Some(Ok(Token { loc, con }));
                };
            } else {
                self.pending.replace(Some(Token {
                    loc,
                    con: TokenContent::Identifier(word),
                }));

                return None;
            }
        }

        self.pending.replace(Some(Token {
            loc,
            con: TokenContent::Identifier(word),
        }));

        None
    }

    fn lex_identifier(&mut self) -> TokenResult {
        let mut word = String::new();
        let mut loc = TokenLoc {
            starts_at: self.current_idx,
            len: 0,
        };

        if let Some(pending) = self.pending.take() {
            loc = pending.loc;

            match &pending.con {
                TokenContent::Identifier(s) => {
                    word = s.to_string();
                }
                _ => {
                    return Err(TokenizerErr::UnexpectedToken);
                }
            }
        };

        while let Some(c) = self.current {
            if c.is_whitespace() {
                break;
            }
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$' | '-' => {
                    word.push(c);
                    self.consume_char();
                }
                _ => {
                    if word.chars().last() == Some('-') {
                        return Err(TokenizerErr::UnexpectedToken);
                    };
                    break;
                }
            }
        }

        loc.len = self.current_idx - loc.starts_at;

        Ok(Token {
            loc,
            con: TokenContent::Identifier(word),
        })
    }

    fn lex_alphabetical_chars(&mut self) -> TokenResult {
        if let Some(token) = self.lex_reserved() {
            return token;
        } else {
            return self.lex_identifier();
        }
    }

    fn lex_anchor(&mut self) -> TokenResult {
        if let Some(c) = self.current {
            let mut loc = TokenLoc {
                starts_at: self.current_idx,
                len: 0,
            };
            if c != '#' {
                return Err(TokenizerErr::InvalidElementIdentifier);
            }

            let mut identifier = String::new();

            identifier.push(c);
            loc.len += 1;

            while let Some(c) = self.advance() {
                if c.is_whitespace() {
                    break;
                } else if c.is_alphabetic() {
                    identifier.push(c);
                } else {
                    break;
                }

                loc.len += 1;
            }

            if identifier.is_empty() {
                return Err(TokenizerErr::EmptyElementIdentifier);
            }

            return Ok(Token {
                loc,
                con: TokenContent::Anchor(identifier),
            });
        } else {
            return Err(TokenizerErr::InvalidElementIdentifier);
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.consume_char();
        self.current
    }

    fn consume_char(&mut self) {
        self.current_idx += 1;

        if self.current_idx == MAX_IDX_VALUE {
            self.full_idx_count += 1;
            self.current_idx = 0;
        }

        self.current = self.itr.next();
    }

    fn set_pending(&mut self, token: Token) -> TokenizationResult {
        assert!(self.pending.replace(Some(token)).is_none());
        Ok(())
    }

    fn set_pending_or_err(&mut self, res: TokenResult) -> TokenizationResult {
        match res {
            Ok(token) => self.set_pending(token),
            Err(err) => Err(err),
        }
    }

    fn tokenize_char(&mut self, c: char) -> TokenizationResult {
        match c {
            'a'..='z' | 'A'..='Z' => {
                let res = self.lex_alphabetical_chars();
                self.set_pending_or_err(res)
            }
            '_' | '$' => {
                let res = self.lex_identifier();
                self.set_pending_or_err(res)
            }
            '0'..='9' => {
                let res = self.lex_number_literal();
                self.set_pending_or_err(res)
            }
            '<' => {
                // ViewElement starting tag
                let loc = TokenLoc {
                    starts_at: self.current_idx,
                    len: 1,
                };
                let con = TokenContent::TagAngleBracketLeft;

                self.consume_char();

                self.set_pending(Token { loc, con })
            }
            '>' => {
                // ViewElement starting tag
                let loc = TokenLoc {
                    starts_at: self.current_idx,
                    len: 1,
                };
                let con = TokenContent::TagAngleBracketRight;

                self.consume_char();

                self.set_pending(Token { loc, con })
            }
            '/' => {
                // Self-closing ViewElement tag
                if let Some('>') = self.advance() {
                    let loc = TokenLoc {
                        starts_at: self.current_idx - 1,
                        len: 2,
                    };
                    let con = TokenContent::TagAngleSelfClosingRight;

                    self.consume_char();

                    self.set_pending(Token { loc, con })
                } else {
                    Err(TokenizerErr::UnexpectedToken)
                }
            }
            '"' => {
                let res = self.lex_string_literal();
                self.set_pending_or_err(res)
            }
            '#' => {
                let res = self.lex_anchor();
                self.set_pending_or_err(res)
            }
            _ => Err(TokenizerErr::UnexpectedToken),
        }
    }

    pub fn next(&mut self) -> Option<TokenResult> {
        while let Some(c) = self.current {
            if c.is_whitespace() {
                self.consume_char();
                continue;
            }

            match self.tokenize_char(c) {
                Ok(..) => {
                    if let Some(token) = self.pending.take() {
                        return Some(Ok(token));
                    } else {
                        panic!("no pending token")
                    }
                }
                Err(err) => {
                    return Some(Err(err));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Tester<'a> {
        name: &'a str,
        expected: Vec<Token>,
        query: &'a str,
    }

    enum TesterErr {
        TokenizerError,
        UnexpectedToken,
    }

    type TesterResult = Result<(), TesterErr>;

    impl<'a> Tester<'a> {
        pub fn new(name: &'a str, expected: Vec<Token>, query: &'a str) -> Self {
            Self {
                name,
                expected,
                query,
            }
        }

        pub fn run(&self) -> TesterResult {
            let mut tokenizer = Tokenizer::new(self.query);
            let mut expected_itr = self.expected.clone().into_iter();

            while let Some(expected) = expected_itr.next() {
                if let Some(token) = tokenizer.next() {
                    match token {
                        Ok(token) => {
                            if token != expected {
                                println!("{}: Failed with unexpected token\n- Expected:\n{:?}\n- Result:\n{:?}", self.name, expected, token);
                                return Err(TesterErr::UnexpectedToken);
                            }
                        }
                        Err(err) => {
                            println!(
                                "{}: Failed with tokenizer error\n- Error:\n{:?}",
                                self.name, err
                            );
                            return Err(TesterErr::TokenizerError);
                        }
                    }
                } else {
                    println!("Tokenizer returned less tokens than expected");
                    return Err(TesterErr::UnexpectedToken);
                }
            }
            println!("{}: Passed", self.name);
            Ok(())
        }
    }

    struct MultiTester<'a> {
        tests: Vec<Tester<'a>>,
    }

    impl<'a> MultiTester<'a> {
        pub fn new() -> Self {
            Self { tests: Vec::new() }
        }

        pub fn add_test(&mut self, test: Tester<'a>) {
            self.tests.push(test);
        }

        pub fn run_all(&mut self) {
            for test in &self.tests {
                assert!(test.run().is_ok());
            }
        }
    }

    #[test]
    fn decimal_digits() {
        assert!(Tester::new(
            "numeric literals",
            vec![Token {
                loc: TokenLoc {
                    starts_at: 0,
                    len: 2,
                },
                con: TokenContent::Literal(TokenLiteral::NumberLiteral("91".to_string())),
            }],
            "91",
        )
        .run()
        .is_ok());
    }

    #[test]
    fn multiple_tokens() {
        assert!(Tester::new(
            "multiple tokens",
            vec![
                Token {
                    loc: TokenLoc {
                        starts_at: 0,
                        len: 1,
                    },
                    con: TokenContent::Identifier("x".to_string()),
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 2,
                        len: 2,
                    },
                    con: TokenContent::Literal(TokenLiteral::NumberLiteral("91".to_string())),
                }
            ],
            "x 91",
        )
        .run()
        .is_ok());
    }

    #[test]
    fn lex_identifier() {
        assert!(Tester::new(
            "identifier",
            vec![Token {
                loc: TokenLoc {
                    starts_at: 0,
                    len: 10,
                },
                con: TokenContent::Identifier("identifier".into()),
            },],
            "identifier",
        )
        .run()
        .is_ok());
    }

    #[test]
    fn lex_identifier_with_dollar_and_underscore() {
        assert!(Tester::new(
            "identifier",
            vec![Token {
                loc: TokenLoc {
                    starts_at: 0,
                    len: 12,
                },
                con: TokenContent::Identifier("$Identifi_er".into()),
            },],
            "$Identifi_er",
        )
        .run()
        .is_ok());
    }

    #[test]
    fn string_literal() {
        assert!(Tester::new(
            "string literal",
            vec![Token {
                loc: TokenLoc {
                    starts_at: 0,
                    len: 14,
                },
                con: TokenContent::Literal(TokenLiteral::StringLiteral(
                    "\"hello, world\"".to_string()
                )),
            }],
            "\"hello, world\"",
        )
        .run()
        .is_ok());
    }

    #[test]
    fn lex_viewtag() {
        let mut tester = MultiTester::new();
        tester.add_test(Tester::new(
            "view self closing tag",
            vec![
                Token {
                    loc: TokenLoc {
                        starts_at: 0,
                        len: 1,
                    },
                    con: TokenContent::TagAngleBracketLeft,
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 1,
                        len: 7,
                    },
                    con: TokenContent::Identifier("Element".into()),
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 8,
                        len: 7,
                    },
                    con: TokenContent::Anchor("#anchor".into()),
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 16,
                        len: 2,
                    },
                    con: TokenContent::TagAngleSelfClosingRight,
                },
            ],
            "<Element#anchor />",
        ));

        tester.add_test(Tester::new(
            "view attribute",
            vec![
                Token {
                    loc: TokenLoc {
                        starts_at: 0,
                        len: 1,
                    },
                    con: TokenContent::TagAngleBracketLeft,
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 1,
                        len: 7,
                    },
                    con: TokenContent::Identifier("Element".into()),
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 8,
                        len: 7,
                    },
                    con: TokenContent::Anchor("#anchor".into()),
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 16,
                        len: 2,
                    },
                    con: TokenContent::TagAngleSelfClosingRight,
                },
            ],
            "<Element#anchor x-attribute-name=\"value\" />",
        ));

        tester.add_test(Tester::new(
            "view attribute",
            vec![
                Token {
                    loc: TokenLoc {
                        starts_at: 0,
                        len: 1,
                    },
                    con: TokenContent::TagAngleBracketLeft,
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 1,
                        len: 7,
                    },
                    con: TokenContent::Identifier("Element".into()),
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 8,
                        len: 7,
                    },
                    con: TokenContent::Anchor("#anchor".into()),
                },
                Token {
                    loc: TokenLoc {
                        starts_at: 16,
                        len: 2,
                    },
                    con: TokenContent::TagAngleSelfClosingRight,
                },
            ],
            "<Element#anchor $sName_A2=\"$doc\"></Element>",
        ));

        tester.run_all();
    }
}

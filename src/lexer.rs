mod iterators;
mod range;
mod tests;

use iterators::*;
use range::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Token {
    Integer(i32),
    Add,
    Sub,
    Div,
    Mul,
}

pub struct Lexer {
    tokens: Vec<Token>,
}
impl Lexer {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn lex(&mut self, raw_text: &str) {
        let mut chars = ByteTracker::new(raw_text.chars().chain("\0".chars()));

        #[derive(Copy, Clone)]
        enum LexerState {
            Start,
            Number,
        }

        let mut state = LexerState::Start;
        let mut slice = MyRange::new();
        let mut current_char = None;
        let mut pulls_attempted = 0;
        const MAX_PULL_ATTEMPS: usize = 5;

        while pulls_attempted < MAX_PULL_ATTEMPS {
            if let Some(CharInfo { glyph, pos, .. }) = current_char {
                match state {
                    LexerState::Start => match glyph {
                        '0'..='9' => {
                            slice = MyRange::start_at_and_include(pos, glyph);
                            state = LexerState::Number;
                        }
                        '+' | '-' | '/' | '*' => {
                            self.tokens.push(match glyph {
                                '+' => Token::Add,
                                '-' => Token::Sub,
                                '*' => Token::Mul,
                                '/' => Token::Div,
                                _ => panic!("should be impossible to reach this state"),
                            });
                        }
                        _ => {}
                    },
                    LexerState::Number => {
                        if glyph.is_numeric() {
                            slice.expand(pos, glyph);
                        } else {
                            let integer_text = slice.as_text(raw_text);
                            let parsed_integer = integer_text.parse().ok().unwrap_or_default();
                            self.tokens.push(Token::Integer(parsed_integer));
                            slice.clear_right();
                            state = LexerState::Start;
                            // allows us to re-run state machine logic with mutated state
                            // and doesn't consume the character
                            continue;
                        }
                    }
                }
            } else {
                pulls_attempted += 1;
            }
            current_char = chars.next();
        }
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

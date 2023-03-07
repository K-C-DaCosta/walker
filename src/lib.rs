pub mod collections;
pub mod lexer;

use lexer::{Lexer, Token};

#[derive(Copy, Clone)]
enum Ast<'a> {
    Add { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    Sub { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    Mul { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    Div { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    LiteralInteger(i64),
}

#[derive(Default)]
pub struct Parser {
    lex: Lexer,
    token_cursor: usize,
}
impl Parser {
    pub fn new() -> Self {
        Self {
            lex: Lexer::new(),
            token_cursor: 0,
        }
    }

    pub fn parse(&mut self, text: &str) -> bool {
        self.lex.lex(text);
        let tokens = self.lex.tokens();

        let state = ParserState {
            tokens,
            token_cursor: 0,
        };

        Self::start(state).unwrap_or_default()
    }

    fn start(state: ParserState) -> Option<bool> {
        Self::multi_div(state)
    }

    fn multi_div(mut state: ParserState) -> Option<bool> {
        let tok = state.get_token()?;
        let expansion_0 = Self::addi_sub(state)?
            && (matches!(tok, Token::Mul) || matches!(tok, Token::Div))
            && Self::multi_div(state)?;
        let expansion_1 = Self::addi_sub(state)?;
        Some(expansion_0 || expansion_1)
    }
    
    fn addi_sub(mut state: ParserState) -> Option<bool> {
        let tok = state.get_token()?;
        Some(
            (matches!(tok, Token::Integer(_))) && Self::addi_sub(state)?
                || matches!(tok, Token::Integer(_)),
        )
    }
}

#[derive(Copy, Clone)]
pub struct ParserState<'a> {
    tokens: &'a [Token],
    token_cursor: usize,
}
impl<'a> ParserState<'a> {
    pub fn get_token(&mut self) -> Option<Token> {
        let token = (self.token_cursor < self.tokens.len()).then(|| self.tokens[self.token_cursor]);
        self.token_cursor += 1;
        token
    }
}

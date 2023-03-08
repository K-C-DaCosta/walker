pub mod collections;
pub mod lexer;

use std::num;

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
}

#[test]
fn parser_sanity_pass() {
    let basic_espression = "(2)+2";
    let mut p = Parser::new();
    let passed = p.parse(basic_espression);
    println!("passed = {passed}");
}

#[test]
fn parser_sanity_fail() {
    let basic_espression = "1+1+1";
    let mut p = Parser::new();
    assert!(!p.parse(basic_espression));
}

impl Parser {
    pub fn new() -> Self {
        Self { lex: Lexer::new() }
    }

    pub fn parse(&mut self, text: &str) -> bool {
        self.lex.lex(text);
        let tokens = self.lex.tokens();

        let mut state = ParserState {
            tokens,
            token_cursor: 0,
        };

        Self::start(&mut state) && state.all_tokens_used()
    }

    fn start(state: &mut ParserState) -> bool {
        Self::arith_expr(state)
    }

    fn arith_expr(state: &mut ParserState) -> bool {
        let epsilon = *state; 
        let mut os = *state;

        let expression_0 = state.accept(&mut os, |tok| matches!(tok, Some(Token::ParenL)))
            && Self::arith_expr(state)
            && state.accept(&mut os, |tok| matches!(tok, Some(Token::ParenR)))
            && Self::arith_expr(state);

        if expression_0 {
            return true; 
        }
        
        *state = os;  
        let expression_1 = Self::multi_div(state);
        if expression_1 {
            return true; 
        }

        *state = epsilon; 
        true
    }

    fn multi_div(state: &mut ParserState) -> bool {
        let mut old_cursor = *state;

        let expansion_0 = Self::addi_sub(state)
            && state.accept(&mut old_cursor, |tok| {
                matches!(tok, Some(Token::Mul | Token::Div))
            })
            && Self::multi_div(state);

        if expansion_0 {
            return true;
        }

        *state = old_cursor;
        let expansion_1 = Self::addi_sub(state);
        if expansion_1 {
            return true;
        }

        false
    }

    fn addi_sub(state: &mut ParserState) -> bool {
        let mut old_state = *state;

        let expansion_0 = state
            .accept(&mut old_state, |tok| matches!(tok, Some(Token::Integer(_))))
            && state.accept(&mut old_state, |tok| {
                matches!(tok, Some(Token::Add | Token::Sub))
            })
            && Self::addi_sub(state);

        if expansion_0 {
            return true;
        }

        *state = old_state;

        let expansion_1 =
            state.accept(&mut old_state, |tok| matches!(tok, Some(Token::Integer(_))));
        if expansion_1 {
            return true;
        }

        false
    }
}

#[derive(Copy, Clone)]
pub struct ParserState<'a> {
    tokens: &'a [Token],
    token_cursor: usize,
}
impl<'a> ParserState<'a> {
    pub fn accept<CB>(&mut self, ss: &mut ParserState<'a>, f: CB) -> bool
    where
        CB: Fn(Option<Token>) -> bool,
    {
        let token = (self.token_cursor < self.tokens.len()).then(|| self.tokens[self.token_cursor]);
        if f(token) {
            *ss = *self;
            self.token_cursor += 1;
            true
        } else {
            false
        }
    }

    pub fn return_token(&mut self) {
        self.token_cursor -= 1;
    }

    pub fn all_tokens_used(&self) -> bool {
        self.token_cursor >= self.tokens.len()
    }
}

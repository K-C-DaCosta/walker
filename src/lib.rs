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
}

#[test]
fn parser_sanity_pass() {
    let basic_espression = "1-1-1-";
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

        Self::start(&mut state) && state.no_more_tokens_left()
    }

    fn start(state: &mut ParserState) -> bool {
        Self::addi_sub(state)
    }

    fn multi_div(state: &mut ParserState) -> bool {
        let expansion_0 = Self::addi_sub(state)
            && state.accept(|tok| matches!(tok, Some(Token::Mul | Token::Div)))
            && Self::multi_div(state);

        if expansion_0 {
            return true;
        }

        let expansion_1 = Self::addi_sub(state);

        if expansion_1 {
            return true;
        }

        false
    }

    fn addi_sub(state: &mut ParserState) -> bool {
        let expansion_0 = state.accept(|tok| matches!(tok, Some(Token::Integer(_))))
            && state.accept(|tok| matches!(tok, Some(Token::Add | Token::Sub)))
            && Self::addi_sub(state);
        if expansion_0 {
            return true;
        }

        // tokens exausted on expansion_0 so return the last token
        state.return_token();


        let expansion_1 = state.accept(|tok| matches!(tok, Some(Token::Integer(_))));
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
    pub fn accept<CB>(&mut self, f: CB) -> bool
    where
        CB: Fn(Option<Token>) -> bool,
    {
        let token = (self.token_cursor < self.tokens.len()).then(|| self.tokens[self.token_cursor]);
        if f(token) {
            self.token_cursor += 1;
            true
        } else {
            false
        }
    }

    pub fn return_token(&mut self){
        self.token_cursor-=1;
    }

    pub fn no_more_tokens_left(&self)->bool{
        self.token_cursor >= self.tokens.len()
    }
}

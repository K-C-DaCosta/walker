use std::char::MAX;

#[derive(Copy, Clone)]
enum Ast<'a> {
    Add { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    Sub { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    Mul { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    Div { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    LiteralInteger(i64),
}

#[derive(Copy, Clone)]
pub struct MyRange {
    lbound: usize,
    ubound: usize,
}

impl MyRange {
    pub fn new() -> Self {
        Self {
            lbound: 0,
            ubound: 0,
        }
    }

    pub fn start_at(lbound: usize) -> Self {
        Self {
            lbound,
            ubound: lbound,
        }
    }
    pub fn start_at_and_include(lbound: usize, c: char) -> Self {
        Self {
            lbound,
            ubound: lbound + c.len_utf8(),
        }
    }

    pub fn expand(&mut self, glyph_idx: usize, glyph: char) {
        self.ubound = self.ubound.max(glyph_idx + glyph.len_utf8());
    }

    pub fn len(&self) -> usize {
        self.ubound - self.lbound
    }

    pub fn clear_left(&mut self) {
        self.ubound = self.lbound;
    }

    pub fn clear_right(&mut self) {
        self.lbound = self.ubound;
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_text<'a>(&self, raw_text: &'a str) -> &'a str {
        &raw_text[self.lbound..self.ubound]
    }
}

impl Default for MyRange {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Token {
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



#[derive(Copy, Clone)]
pub struct CharInfo {
    pub glyph: char,
    pub pos: usize,
    pub aux: HistoryState,
}

#[derive(Clone)]

pub struct History<const N: usize> {
    values: [usize; N],
    value_cursor: usize,
    len: usize,
}
impl<const N: usize> History<N> {
    pub fn new() -> Self {
        Self {
            values: [!0; N],
            value_cursor: 0,
            len: 0,
        }
    }

    pub fn prev(&self, offset: usize) -> usize {
        self.values[(self.value_cursor + N - offset) % N]
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn push(&mut self, val: usize) {
        self.values[self.value_cursor] = val;
        self.value_cursor = (self.value_cursor + 1) % N;
        self.len = (self.len + 1).max(N);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Default for History<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct ByteTracker<UnicodeIter> {
    char_iter: UnicodeIter,
    glyph_pos: usize,
    position_lbound: History<16>,
    position_ubound: History<16>,
}

#[derive(Copy, Clone)]
pub struct HistoryState {
    position_lbound: *const History<16>,
    position_ubound: *const History<16>,
}
impl HistoryState {
    pub fn position_ubound(&self) -> &History<16> {
        unsafe { self.position_ubound.as_ref().unwrap() }
    }
    pub fn position_lbound(&self) -> &History<16> {
        unsafe { self.position_lbound.as_ref().unwrap() }
    }
}

impl<UnicodeIter> ByteTracker<UnicodeIter>
where
    UnicodeIter: Clone + Iterator<Item = char>,
{
    pub fn new(char_iter: UnicodeIter) -> Self {
        Self {
            char_iter,
            glyph_pos: 0,
            position_lbound: History::new(),
            position_ubound: History::new(),
        }
    }
}

impl<UnicodeIter> Iterator for ByteTracker<UnicodeIter>
where
    UnicodeIter: Iterator<Item = char> + Clone,
{
    type Item = CharInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let char_iter = &mut self.char_iter;
        let position_hist = &mut self.position_lbound;
        let glyph_len_hist = &mut self.position_ubound;
        let glyph_pos_ref = &mut self.glyph_pos;

        char_iter.next().map(|glyph| {
            let glyph_len = glyph.len_utf8();
            let glyph_pos = *glyph_pos_ref;

            position_hist.push(glyph_pos + glyph_len);
            glyph_len_hist.push(glyph_len);
            *glyph_pos_ref += glyph_len;

            CharInfo {
                glyph,
                pos: glyph_pos,
                aux: HistoryState {
                    position_lbound: position_hist,
                    position_ubound: glyph_len_hist,
                },
            }
        })
    }
}




mod lexer_tests {
    #[test]
    pub fn lexer_sanity_test_0() {
        use crate::*;
        let raw_text = "123*324";

        let mut lexer = Lexer::new();
        lexer.lex(raw_text);
        assert_eq!(
            vec![Token::Integer(123), Token::Mul, Token::Integer(324)],
            lexer.tokens
        );
    }
    #[test]
    pub fn lexer_sanity_test_1() {
        use crate::*;
        let raw_text = "123+324";

        let mut lexer = Lexer::new();
        lexer.lex(raw_text);
        assert_eq!(
            vec![Token::Integer(123), Token::Add, Token::Integer(324)],
            lexer.tokens
        );
    }
    #[test]
    pub fn lexer_sanity_test_2() {
        use crate::*;
        let raw_text = "123-324";

        let mut lexer = Lexer::new();
        lexer.lex(raw_text);
        assert_eq!(
            vec![Token::Integer(123), Token::Sub, Token::Integer(324)],
            lexer.tokens
        );
    }
}
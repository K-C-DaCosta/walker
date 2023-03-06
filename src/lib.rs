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

    pub fn expand(&mut self, new_ubound: usize) {
        self.ubound = self.ubound.max(new_ubound);
    }

    pub fn len(&self) -> usize {
        self.ubound - self.lbound
    }

    pub fn clear(&mut self) {
        self.ubound = self.lbound;
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
        let chars = ByteTracker::new(raw_text.chars().chain("\0".chars()));

        #[derive(Copy, Clone)]
        enum LexerState {
            Start,
            Number,
        }

        let mut state = LexerState::Start;
        let mut slice = MyRange::new();

        for CharInfo { glyph, pos } in chars {
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
                LexerState::Number => {}
            }
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
}

#[derive(Clone)]

pub struct History<const N:usize> {
    values: [usize; N],
    value_cursor: usize,
    len:usize, 
}
impl <const N:usize> History<N> {
    pub fn new() -> Self {
        Self {
            values: [!0; N],
            value_cursor: 0,
            len: 0, 
        }
    }

    pub fn clear(&mut self){
        self.len = 0; 
    }

    pub fn push(&mut self, val:usize){
        self.values[self.value_cursor] = val; 
        self.value_cursor = (self.value_cursor + 1) % N; 
        self.len = (self.len +1).max(N);
    }
    
    pub fn len(&self)->usize{
        self.len
    }

    pub fn is_empty(&self)->bool{
        self.len() == 0 
    }
}

pub struct ByteTracker<UnicodeIter> {
    char_iter: UnicodeIter,
    glyph_pos: usize,
    history: History<16>,
}

impl<UnicodeIter> ByteTracker<UnicodeIter>
where
    UnicodeIter: Clone + Iterator<Item = char>,
{
    pub fn new(char_iter: UnicodeIter) -> Self {
        Self {
            char_iter,
            glyph_pos: 0,
            history: History::new(),
        }
    }
}

impl<UnicodeIter> Iterator for ByteTracker<UnicodeIter>
where
    UnicodeIter: Iterator<Item = char> + Clone,
{
    type Item = CharInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.char_iter.next().map(|c| {
            let glyph_offset = c.len_utf8();
            let glyph_pos = self.glyph_pos;
            self.history.push(glyph_pos);


            self.glyph_pos += glyph_offset;
            CharInfo {
                glyph: c,
                pos: glyph_pos,
            }
        })
    }
}

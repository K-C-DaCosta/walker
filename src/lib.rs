#[derive(Copy, Clone)]
enum Ast<'a> {
    Add { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    Sub { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    Mul { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
    Div { lhs: &'a Ast<'a>, rhs: &'a Ast<'a> },
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

impl Default for MyRange{
    fn default() -> Self {
        Self::new()
    }
}


enum Tokens {
    Integer(i32),
    Add(MyRange),
    Sub(MyRange),
    Div(MyRange),
    Mul(MyRange),
}

pub struct Lexer {
    tokens: Vec<Tokens>,
}
impl Lexer {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub fn lex(&mut self, raw_text: &str) {
        // raw_text.chars()
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
pub struct ByteTracker<UnicodeIter> {
    char_iter: UnicodeIter,
    glyph_pos: usize,
}

impl<UnicodeIter> ByteTracker<UnicodeIter>
where
    UnicodeIter: Clone + Iterator<Item = char>,
{
    pub fn new(char_iter: UnicodeIter) -> Self {
        Self {
            char_iter,
            glyph_pos: 0,
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
            self.glyph_pos += glyph_offset;
            CharInfo {
                glyph: c,
                pos: glyph_pos,
            }
        })
    }
}
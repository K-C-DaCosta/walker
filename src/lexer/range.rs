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
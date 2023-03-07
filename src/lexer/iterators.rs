use crate::collections::*; 

#[derive(Copy, Clone)]
pub struct CharInfo {
    pub glyph: char,
    pub pos: usize,
    pub aux: HistoryState,
}

#[derive(Clone)]
pub struct ByteTracker<UnicodeIter> {
    char_iter: UnicodeIter,
    glyph_pos: usize,
    position_lbound: History<16,usize>,
    position_ubound: History<16,usize>,
}

#[derive(Copy, Clone)]
pub struct HistoryState {
    position_lbound: *const History<16,usize>,
    position_ubound: *const History<16,usize>,
}
impl HistoryState {
    pub fn position_ubound(&self) -> &History<16,usize> {
        unsafe { self.position_ubound.as_ref().unwrap() }
    }
    pub fn position_lbound(&self) -> &History<16,usize> {
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
        let lbound_hist = &mut self.position_lbound;
        let ubound_hist = &mut self.position_ubound;
        let glyph_pos_ref = &mut self.glyph_pos;

        char_iter.next().map(|glyph| {
            let glyph_len = glyph.len_utf8();
            let glyph_pos = *glyph_pos_ref;

            lbound_hist.push(glyph_pos + glyph_len);
            ubound_hist.push(glyph_len);
            *glyph_pos_ref += glyph_len;

            CharInfo {
                glyph,
                pos: glyph_pos,
                aux: HistoryState {
                    position_lbound: lbound_hist,
                    position_ubound: ubound_hist,
                },
            }
        })
    }
}
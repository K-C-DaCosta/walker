#[derive(Clone)]

pub struct History<const N: usize, T> {
    values: [T; N],
    value_cursor: usize,
    len: usize,
}
impl<const N: usize, T> History<N, T>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        Self {
            values: [T::default(); N],
            value_cursor: 0,
            len: 0,
        }
    }

    pub fn prev(&self, offset: usize) -> T {
        self.values[(self.value_cursor + N - offset) % N]
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn push(&mut self, val: T) {
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

impl<const N: usize, T> Default for History<N, T> 
where T:Copy+Default
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct Queue<'a, T: PartialEq> {
    pub data: &'a [T],
    pub cursor: usize,
}

impl<'a, T: PartialEq> Queue<'a, T> {
    pub fn new(data: &[T]) -> Queue<T> {
        Queue { data, cursor: 0 }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn empty(&self) -> bool {
        self.cursor >= self.data.len()
    }

    pub fn head(&self) -> &T {
        &self.data[self.cursor]
    }

    pub fn back(&self, back: usize) -> &T {
        &self.data[self.cursor - back]
    }

    pub fn peak(&self) -> Option<&T> {
        self.data.get(self.cursor)
    }

    pub fn pop(&mut self) -> Option<&T> {
        match self.data.get(self.cursor) {
            Some(t) => {
                self.cursor += 1;
                Some(t)
            }
            None => None,
        }
    }

    pub fn pop_while(&mut self, mut f: impl FnMut(&T) -> bool) -> &[T] {
        let start = self.cursor;
        while self.cursor < self.data.len() && f(&self.data[self.cursor]) {
            self.cursor += 1;
        }
        &self.data[start..self.cursor]
    }

    pub fn pop_until(&mut self, pattern: &[T]) -> &[T] {
        let start = self.cursor;
        while self.cursor < self.data.len() && !self.data[self.cursor..].starts_with(pattern) {
            self.cursor += 1;
        }
        &self.data[start..self.cursor]
    }
}

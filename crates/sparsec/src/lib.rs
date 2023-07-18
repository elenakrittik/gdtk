use std::iter::Iterator;

pub struct Sparsec<'a> {
    pub inner: std::str::Chars<'a>,
}

impl<'a> Sparsec<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: input.chars(),
        }
    }
}

impl<'a> Iterator for Sparsec<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

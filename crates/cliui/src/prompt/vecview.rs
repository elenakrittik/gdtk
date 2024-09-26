use std::ops::Range;

pub struct VecView<T> {
    items: Vec<T>,
    current_idx: usize,
    length: usize,
    drag_limit: usize,
    range: Range<usize>,
}

pub struct ViewData<'a, T> {
    pub items: &'a [T],
}

impl<T> VecView<T> {
    pub fn new(items: Vec<T>, length: usize, drag_limit: usize) -> Self {
        Self {
            items,
            range: 0..length,
            length,
            drag_limit,
            current_idx: 0,
        }
    }

    #[inline]
    pub fn current_item_index(&self) -> usize {
        self.current_idx
    }

    #[inline]
    pub fn range_start(&self) -> usize {
        self.range.start
    }

    #[inline]
    pub fn range_end(&self) -> usize {
        self.range.end
    }

    #[inline]
    pub fn consume_item(mut self, idx: usize) -> T {
        self.items.swap_remove(idx)
    }

    pub fn move_up(&mut self) {
        if self.current_idx > 0 {
            self.current_idx -= 1;

            let range_end = if self.range_end() == self.items.len() {
                self.range_end() - 1
            } else {
                self.range_end()
            };

            if range_end - self.current_idx > self.drag_limit {
                self.move_range_up();
            }
        }

        debug_assert!(self.range.end - self.range.start == self.length);
    }

    fn move_range_up(&mut self) {
        if self.range_start() > 0 {
            self.range.start -= 1;
            self.range.end -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let last_items_idx = self.items.len().saturating_sub(1);

        if self.current_idx < (last_items_idx) {
            self.current_idx += 1;

            if (self.current_idx - self.range_start()) > self.drag_limit {
                self.move_range_down();
            }
        }

        debug_assert!(self.range.end - self.range.start == self.length);
    }

    fn move_range_down(&mut self) {
        if self.range_end() < self.items.len() {
            self.range.start += 1;
            self.range.end += 1;
        }
    }

    pub fn view(&self) -> ViewData<T> {
        // number of items can be less than the desired length, so we account for that
        let end = std::cmp::min(self.items.len(), self.range_end());

        ViewData {
            items: &self.items[self.range_start()..end],
        }
    }
}

impl<T> std::ops::Deref for VecView<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

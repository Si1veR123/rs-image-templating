use super::Filter;

#[derive(Default)]
pub struct TranslateFilter {
    pub x: isize,
    pub y: isize
}

impl<T> Filter<T> for TranslateFilter {
    fn filter_transform(&self, x: usize, y: usize) -> (usize, usize) {
        (x.wrapping_add_signed(-self.x), y.wrapping_add_signed(-self.y))
    }
}

use crate::AlphaPixel;

pub mod transform;

pub trait Filter<T> {
    fn filter_pixel(&self, pixel: AlphaPixel<T>) -> AlphaPixel<T> {
        pixel
    }

    fn filter_transform(&self, x: usize, y: usize) -> (usize, usize) {
        (x, y)
    }
}

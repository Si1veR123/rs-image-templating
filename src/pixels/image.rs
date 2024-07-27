use std::mem::size_of;

use super::pixel::{AlphaPixel, PixelChannel};

#[derive(Debug, Clone)]
pub struct Image<T: PixelChannel> {
    pixels: Vec<AlphaPixel<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: PixelChannel> Image<T> {
    pub const fn bit_depth(&self) -> usize {
        size_of::<T>()
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<AlphaPixel<T>> {
        self.pixels.get(x*self.width+y).cloned()
    }

    pub fn from_pixels(pixels: Vec<AlphaPixel<T>>, width: usize) -> Self {
        let height = pixels.len() / 3;
        Self { pixels, width, height }
    }
}

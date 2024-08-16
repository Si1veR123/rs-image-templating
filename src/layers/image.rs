use crate::{Filter, Image, AlphaPixel, PixelChannel, Rect, Layer};

#[derive(Default)]
pub struct ImageLayer<T: PixelChannel> {
    pub filters: Vec<Box<dyn Filter<T>>>,
    pub im: Image<T>,
    pub x: usize,
    pub y: usize
}

impl<T: PixelChannel> ImageLayer<T> {
    pub fn new(im: Image<T>, x: usize, y: usize) -> Self {
        Self { filters: vec![], im, x, y }
    }
}

impl<T: PixelChannel> Layer<T> for ImageLayer<T> {
    fn get_rect(&self) -> Rect {
        Rect { x: self.x, y: self.y, width: self.im.get_width(), height: self.im.get_height() }
    }

    fn get_filters(&self) -> &[Box<dyn Filter<T>>] {
        &self.filters
    }

    fn unfiltered_pixel_at_unchecked(&self, x: usize, y: usize) -> AlphaPixel<T> {
        self.im.pixel_at(x-self.x, y-self.y).unwrap()
    }
}

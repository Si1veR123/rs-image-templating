use crate::{Filter, Layer, AlphaPixel, PixelChannel, Rect};


pub struct RectangleLayer<T> {
    pub filters: Vec<Box<dyn Filter<T>>>,
    pub fill: AlphaPixel<T>,
    pub rect: Rect
}

impl<T> RectangleLayer<T> {
    pub fn new(fill: AlphaPixel<T>, rect: Rect) -> Self {
        Self { filters: vec![], fill, rect }
    }
}

impl<T: PixelChannel> Layer<T> for RectangleLayer<T> {
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn get_filters(&self) -> &[Box<dyn Filter<T>>] {
        &self.filters
    }

    fn unfiltered_pixel_at_unchecked(&self, _x: usize, _y: usize) -> AlphaPixel<T> {
        self.fill
    }
}

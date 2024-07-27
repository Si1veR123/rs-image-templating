use crate::{filters::Filter, layers::Layer, pixels::pixel::{AlphaPixel, PixelChannel}, rect::Rect};


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

    fn get_filters(&self) -> &[Box<dyn crate::filters::Filter<T>>] {
        &self.filters
    }

    fn unfiltered_pixel_at(&self, x: usize, y: usize) -> Option<AlphaPixel<T>> {
        if self.rect.contains(x, y) {
            Some(self.fill)
        } else {
            None
        }
    }
}

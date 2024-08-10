use crate::{filters::Filter, pixels::{image::Image, pixel::{AlphaPixel, PixelChannel}}, rect::Rect};
use super::Layer;

#[derive(Default)]
pub struct ImageLayer<T: PixelChannel> {
    pub filters: Vec<Box<dyn Filter<T>>>,
    pub im: Image<T>,
    pub x: usize,
    pub y: usize
}

impl<T: PixelChannel> Layer<T> for ImageLayer<T> {
    fn get_rect(&self) -> Rect {
        Rect { x: self.x, y: self.y, width: self.im.width, height: self.im.height }
    }

    fn get_filters(&self) -> &[Box<dyn Filter<T>>] {
        &self.filters
    }

    fn unfiltered_pixel_at_unchecked(&self, x: usize, y: usize) -> AlphaPixel<T> {
        self.im.pixel_at(x-self.x, y-self.y).unwrap()
    }
}

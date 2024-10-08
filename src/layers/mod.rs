use crate::{Filter, AlphaPixel, PixelChannel, Rect};

pub mod image;
pub mod shapes;
pub mod text;

pub trait Layer<T: PixelChannel> {
    /// Get a bounding `Rect` relative to top left of the canvas.
    /// 
    /// Only pixels in this `Rect` will be drawn
    fn get_rect(&self) -> Rect;

    /// Return a slice of filters on this layer
    fn get_filters(&self) -> &[Box<dyn Filter<T>>];

    /// Get the pixel at a canvas location, after it has been filtered
    fn filtered_pixel_at(&self, x: usize, y: usize) -> Option<AlphaPixel<T>> {
        let mut transformed_coord = (x, y);
        let filters = self.get_filters();
        for filter in filters {
            transformed_coord = filter.filter_transform(transformed_coord.0, transformed_coord.1);
        }

        let mut pixel = self.unfiltered_pixel_at(transformed_coord.0, transformed_coord.1)?;
        for filter in filters {
            pixel = filter.filter_pixel(pixel)
        }
        Some(pixel)
    }

    /// Get the pixel at a canvas location, before it has been filtered.
    /// 
    /// Includes a check that it is within the layer's bounds, and reutrns None if it isn't.
    fn unfiltered_pixel_at(&self, x: usize, y: usize) -> Option<AlphaPixel<T>> {
        if self.get_rect().contains(x, y) {
            Some(self.unfiltered_pixel_at_unchecked(x, y))
        } else {
            None
        }
    }

    /// Get the pixel at a canvas location, before it has been filtered, and assuming it is within the bounding `Rect`.
    /// 
    /// This will panic if the coordinates are not in bounds.
    /// 
    /// Use `unfiltered_pixel_at` if the coordinate may not be in bounds.
    fn unfiltered_pixel_at_unchecked(&self, x: usize, y: usize) -> AlphaPixel<T>;
}

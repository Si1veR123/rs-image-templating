use crate::{filters::Filter, pixels::{image::Image, pixel::{AlphaPixel, PixelChannel}}, rect::Rect};

pub mod image;
pub mod shapes;

pub trait Layer<T: PixelChannel> {
    /// Get a bounding Rect relative to top left of the canvas
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

    /// Get the pixel at a canvas location, before it has been filtered
    fn unfiltered_pixel_at(&self, x: usize, y: usize) -> Option<AlphaPixel<T>>;

    /// Convert a layer into an `Image`, which is a list of pixels
    fn rasterize(&self) -> Image<T> {
        let rect = self.get_rect();
        
        let mut pixels = Vec::with_capacity(rect.width*rect.height);
        for row in 0..rect.height {
            for col in 0..rect.width {
                pixels.push(self.unfiltered_pixel_at(col+rect.x, row+rect.y).unwrap_or(AlphaPixel::default()));
            }
        }

        Image::from_pixels(pixels, rect.width)
    }
}

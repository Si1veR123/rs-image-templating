use crate::AlphaPixel;

pub mod transform;
pub mod brightness;

/// This trait is used for types that can be added to layers to filter them.
pub trait Filter<T> {
    /// This method is used to filter the colour of an image. 
    /// 
    /// It takes a pixel to filter, and returns the filtered pixel.
    fn filter_pixel(&self, pixel: AlphaPixel<T>) -> AlphaPixel<T> {
        pixel
    }

    /// This method is used to filter the location that the pixel is sampled from.
    /// 
    /// It takes the coordinate of the pixel that is being sampled, and returns
    /// the transformed coordinate to sample the pixel from.
    /// 
    /// This means that the actual transformations that are applied to the layer
    /// are inverted.
    fn filter_transform(&self, x: usize, y: usize) -> (usize, usize) {
        (x, y)
    }
}

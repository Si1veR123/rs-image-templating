use std::path::Path;
use bytemuck::must_cast_slice;
use image::{error::{ParameterError, ParameterErrorKind}, save_buffer_with_format, ImageFormat};
use num::Integer;
use thiserror::Error;
use super::{blending::BlendingMethod, pixel::{AlphaPixel, PixelChannel}};

#[derive(Debug, Error, PartialEq)]
pub enum NewImageError {
    #[error("Width is incorrect")]
    IncorrectWidth,
    #[error("Width is 0, but buffer isn't zero-length")]
    ZeroWidth
}

#[derive(Debug, Clone)]
/// A collection of `AlphaPixel`s that represent an image. This is stored in a `Vec`.
pub struct Image<T: PixelChannel> {
    pixels: Vec<AlphaPixel<T>>,
    width: usize,
    height: usize,
}

impl<T: PixelChannel> Default for Image<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PixelChannel> Image<T> {
    pub fn new() -> Self {
        Self { pixels: vec![], width: 0, height: 0 }
    }

    pub fn new_with_fill(fill: AlphaPixel<T>, width: usize, height: usize) -> Self {
        let pixels = std::iter::repeat(fill).take(width*height).collect();
        Self { pixels, width, height }
    }

    pub fn from_pixels(pixels: Vec<AlphaPixel<T>>, width: usize) -> Result<Self, NewImageError> {
        if width == 0 {
            if pixels.is_empty() {
                return Ok(Self::new());
            } else {
                return Err(NewImageError::ZeroWidth)
            }
        }

        let (height, rem) = pixels.len().div_rem(&width);
        if rem != 0 {
            Err(NewImageError::IncorrectWidth)
        } else {
            Ok(Self { pixels, width, height })
        }
    }

    pub fn from_function<F: FnMut(usize, usize) -> AlphaPixel<T>>(width: usize, height: usize, mut function: F) -> Self {
        let mut pixels = Vec::with_capacity(width*height);
        for row in 0..height {
            for col in 0..width {
                pixels.push(function(col, row))
            }
        }
        Self { pixels, width, height }
    }

    pub fn get_pixels(&self) -> &[AlphaPixel<T>] {
        &self.pixels
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn index_of_unchecked(&self, x: usize, y: usize) -> usize {
        self.width*y + x
    }

    fn index_of(&self, x: usize, y: usize) -> Option<usize> {
        if self.contains(x, y) {
            Some(self.index_of_unchecked(x, y))
        } else {
            None
        }
    }

    pub fn row(&self, y: usize) -> Option<&[AlphaPixel<T>]> {
        // Last index may not be in the image, but this is okay as the range is exclusive.
        let range = self.index_of(0, y)?..self.index_of_unchecked(0, y+1);
        self.pixels.get(range)
    }

    pub fn row_mut(&mut self, y: usize) -> Option<&mut [AlphaPixel<T>]> {
        let range = self.index_of(0, y)?..self.index_of(0, y+1)?;
        self.pixels.get_mut(range)
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<AlphaPixel<T>> {
        self.pixels.get(self.index_of(x, y)?).copied()
    }

    pub fn pixel_at_mut(&mut self, x: usize, y: usize) -> Option<&mut AlphaPixel<T>> {
        let idx = self.index_of(x, y)?;
        self.pixels.get_mut(idx)
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// If `None` is returned, then the coordinate is not in the image bounds.
    pub fn draw_subimage(&mut self, image: &Image<T>, x: usize, y: usize, blend: BlendingMethod<T>) -> Option<()> {
        let subim_width = (x+image.width).min(self.width) - x;
        let subim_height = (y+image.height).min(self.height) - y;

        for row in 0..subim_height {
            let slice = self.index_of(x, y+row)?..self.index_of_unchecked(x+subim_width, y+row);
            let src_row = &image.row(row).unwrap()[0..subim_width];
            
            self.pixels[slice].iter_mut()
                .zip(src_row.iter())
                .for_each(|(dest, src)| *dest = blend.blend(*dest, *src));
        }

        Some(())
    }

    /// Fails if width or height cannot fit into a `u32`
    /// 
    /// Color type is determined by `color_type` method in AlphaPixel
    pub fn save<P: AsRef<Path>>(&self, path: P, format: ImageFormat) -> image::ImageResult<()> {
        let width: u32 = self.width.try_into()
            .map_err(|_| image::ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)))?;
        let height: u32 = self.height.try_into()
            .map_err(|_| image::ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)))?;

        save_buffer_with_format(path, self.as_ref(), width, height, AlphaPixel::<T>::color_type(), format)
    }
}

impl<T: PixelChannel> AsRef<[u8]> for Image<T> {
    fn as_ref(&self) -> &[u8] {
        must_cast_slice(&self.pixels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rgba;

    fn generation_function(x: usize, y: usize) -> AlphaPixel<u8> {
        let blue = 255;
        let alpha = 255;
        rgba!(x as u8, y as u8, blue, alpha)
    }

    fn create_test_image() -> Image<u8> {
        Image::from_function(255, 255, generation_function)
    }

    #[test]
    /// This tests both `image.pixel_at(...)` and `Image::from_function(...)`
    fn pixel_at() {
        let image = create_test_image();

        for row in 0..image.get_height() {
            for col in 0..image.get_width() {
                assert_eq!(generation_function(col, row), image.pixel_at(col, row).unwrap())
            }
        }
    }

    #[test]
    fn from_pixels_fail() {
        let pixels = vec![AlphaPixel::<u8>::black(); 10];

        let image_error_incorrect_width = Image::from_pixels(pixels.clone(), 3);
        assert!(image_error_incorrect_width.is_err());
        assert_eq!(image_error_incorrect_width.unwrap_err(), NewImageError::IncorrectWidth);

        let image_error_zero_width = Image::from_pixels(pixels, 0);
        assert!(image_error_zero_width.is_err());
        assert_eq!(image_error_zero_width.unwrap_err(), NewImageError::ZeroWidth);
    }

    #[test]
    fn from_pixels_valid() {
        let valid_image = Image::<u8>::from_pixels(vec![], 0);
        assert!(valid_image.is_ok());

        let valid_pixels: Vec<AlphaPixel<u8>> = (0..100).map(|i| AlphaPixel { r: i, g: 100 - i, b: 255, a: 255 }).collect();
        let valid_image = Image::from_pixels(valid_pixels, 10);
        assert!(valid_image.is_ok());
    }

    #[test]
    fn contains() {
        let image = Image::from_function(100, 50, |_, _| AlphaPixel::<u8>::black());
        assert!(image.contains(0, 0));
        assert!(image.contains(10, 10));
        assert!(image.contains(99, 49));
        assert!(!image.contains(150, 10));
        assert!(!image.contains(150, 150));
        assert!(!image.contains(100, 50));
    }

    #[test]
    fn get_row() {
        let mut image = create_test_image();

        image.row_mut(99).unwrap().fill(AlphaPixel::green());

        assert_eq!(image.row(99).unwrap(), [AlphaPixel::green(); 255]);
        assert_eq!(image.row(0).unwrap(), (0..255).map(|i| AlphaPixel { r: i, g: 0, b: 255, a: 255 }).collect::<Vec<AlphaPixel<u8>>>());
        assert_eq!(image.row(50).unwrap(), (0..255).map(|i| AlphaPixel { r: i, g: 50, b: 255, a: 255 }).collect::<Vec<AlphaPixel<u8>>>());
    }

    #[test]
    fn draw_subimage() {
        let mut background_image = Image::<u8>::new_with_fill(AlphaPixel::red(), 100, 100);
        let subimage = Image::new_with_fill(AlphaPixel::blue(), 30, 20);
        background_image.draw_subimage(&subimage, 50, 20, BlendingMethod::Replace);

        assert_eq!(background_image.pixel_at(0, 0).unwrap(), AlphaPixel::red());

        assert_eq!(background_image.pixel_at(49, 30).unwrap(), AlphaPixel::red());
        assert_eq!(background_image.pixel_at(50, 30).unwrap(), AlphaPixel::blue());
        assert_eq!(background_image.pixel_at(79, 30).unwrap(), AlphaPixel::blue());
        assert_eq!(background_image.pixel_at(80, 30).unwrap(), AlphaPixel::red());

        assert_eq!(background_image.pixel_at(65, 19).unwrap(), AlphaPixel::red());
        assert_eq!(background_image.pixel_at(65, 20).unwrap(), AlphaPixel::blue());
        assert_eq!(background_image.pixel_at(65, 39).unwrap(), AlphaPixel::blue());
        assert_eq!(background_image.pixel_at(65, 40).unwrap(), AlphaPixel::red());

        assert_eq!(background_image.pixel_at(99, 99).unwrap(), AlphaPixel::red());
    }
}

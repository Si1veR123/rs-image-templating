use std::path::Path;
use bytemuck::must_cast_slice;
use image::{error::{ParameterError, ParameterErrorKind}, save_buffer_with_format, ImageFormat};
use num::Integer;
use thiserror::Error;
use super::{blending::BlendingMethod, pixel::{AlphaPixel, PixelChannel}};

#[derive(Debug, Error)]
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
    pub width: usize,
    pub height: usize,
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

    fn index_of_unchecked(&self, x: usize, y: usize) -> usize {
        self.width*y + x
    }

    fn index_of(&self, x: usize, y: usize) -> Option<usize> {
        if self.contains_coord(x, y) {
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

    pub fn contains_coord(&self, x: usize, y: usize) -> bool {
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

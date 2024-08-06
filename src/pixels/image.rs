use std::path::Path;
use bytemuck::must_cast_slice;
use image::{error::{ParameterError, ParameterErrorKind}, save_buffer_with_format, ImageError, ImageFormat};
use num::Integer;
use super::{blending::BlendingMethod, pixel::{AlphaPixel, PixelChannel}};


#[derive(Debug, Clone)]
/// A collection of `AlphaPixel`s that represent an image. This is stored in a `Vec`.
pub struct Image<T: PixelChannel> {
    pixels: Vec<AlphaPixel<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: PixelChannel> Image<T> {
    fn index_of(&self, x: usize, y: usize) -> usize {
        self.width*y + x
    }

    pub fn row(&self, y: usize) -> Option<&[AlphaPixel<T>]> {
        let range = self.index_of(0, y)..self.index_of(0, y+1);
        self.pixels.get(range)
    }

    pub fn row_mut(&mut self, y: usize) -> Option<&mut [AlphaPixel<T>]> {
        let range = self.index_of(0, y)..self.index_of(0, y+1);
        self.pixels.get_mut(range)
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<AlphaPixel<T>> {
        self.pixels.get(self.index_of(x, y)).copied()
    }

    pub fn pixel_at_mut(&mut self, x: usize, y: usize) -> Option<&mut AlphaPixel<T>> {
        let idx = self.index_of(x, y);
        self.pixels.get_mut(idx)
    }

    pub fn coord_contains(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn draw_subimage(&mut self, image: &Image<T>, x: usize, y: usize, blend: BlendingMethod<T>) {
        // BOUNDS CHECK

        let subim_width = (x+image.width).min(self.width) - x;
        let subim_height = (y+image.height).min(self.height) - y;

        for row in 0..subim_height {
            let slice = self.index_of(x, y+row)..self.index_of(x+subim_width, y+row);
            let src_row = &image.row(row).unwrap()[0..subim_width];
            
            self.pixels[slice.clone()].iter_mut()
                .zip(src_row.iter())
                .for_each(|(dest, src)| *dest = blend.blend(*dest, *src));
        }
    }

    /// Will return Err(()) if `pixels.len() % width != 0`. This ensures that `width` and `height` are correct.
    pub fn from_pixels(pixels: Vec<AlphaPixel<T>>, width: usize) -> Result<Self, ()> {
        let (height, rem) = pixels.len().div_rem(&width);
        if rem != 0 {
            Err(())
        } else {
            Ok(Self { pixels, width, height })
        }
    }

    /// Fails if width or height cannot fit into a `u32`
    /// 
    /// Color type is determined by `color_type` method in AlphaPixel
    pub fn save<P: AsRef<Path>>(&self, path: P, format: ImageFormat) -> image::ImageResult<()> {
        let width: u32 = self.width.try_into()
            .map_err(|_| ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)))?;
        let height: u32 = self.height.try_into()
            .map_err(|_| ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)))?;

        save_buffer_with_format(path, self.as_ref(), width, height, AlphaPixel::<T>::color_type(), format)
    }
}

impl<T: PixelChannel> AsRef<[u8]> for Image<T> {
    fn as_ref(&self) -> &[u8] {
        must_cast_slice(&self.pixels)
    }
}

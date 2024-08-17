use bytemuck::must_cast_slice;
use num::Integer;
use thiserror::Error;
use crate::{BlendingMethod, AlphaPixel, PixelChannel};

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
    pub fn get_pixels(&self) -> &[AlphaPixel<T>] {
        &self.pixels
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Create a new empty image, with zero width and height.
    /// 
    /// ```
    /// use image_template::Image;
    /// let image: Image<u8> = Image::new();
    /// ```
    pub fn new() -> Self {
        Self { pixels: vec![], width: 0, height: 0 }
    }

    /// Create a new image, filled with `fill`.
    /// 
    /// ```
    /// use image_template::{Image, rgba};
    /// let image: Image<u8> = Image::new_with_fill(rgba!(255, 0, 0, 255), 10, 10);
    /// ```
    pub fn new_with_fill(fill: AlphaPixel<T>, width: usize, height: usize) -> Self {
        let pixels = vec![fill; width*height];
        Self { pixels, width, height }
    }

    /// Create a new image, from a [`Vec`] of `AlphaPixel<T>`.
    /// 
    /// ```
    /// use image_template::{AlphaPixel, Image};
    /// 
    /// let pixels = vec![AlphaPixel::black(), AlphaPixel::red(), AlphaPixel::blue(), AlphaPixel::white()];
    /// let image: Image<u8> = Image::from_pixels(pixels, 2).unwrap();
    /// assert_eq!(image.get_height(), 2);
    /// assert_eq!(image.get_width(), 2);
    /// ```
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

    /// Create an image from a function that maps coordinates to pixels.
    /// 
    /// `function` is a type implementing [`FnMut`] that can be called with an x and y coordinate, and return a pixel.
    /// 
    /// ```
    /// use image_template::{Image, AlphaPixel};
    /// 
    /// let generate = |x, y| AlphaPixel { r: x as u8, g: y as u8, b: 255, a: 255 };
    /// let image: Image<u8> = Image::from_function(10, 10, generate);
    /// ```
    pub fn from_function<F: FnMut(usize, usize) -> AlphaPixel<T>>(width: usize, height: usize, mut function: F) -> Self {
        let mut pixels = Vec::with_capacity(width*height);
        for row in 0..height {
            for col in 0..width {
                pixels.push(function(col, row))
            }
        }
        Self { pixels, width, height }
    }

    /// Get the index into the collection of pixels for a given coordinate.
    /// 
    /// This does NOT check whether the coordinate is actually within the image's bounds.
    /// `Image::index_of` includes a bounds check.
    /// 
    /// ```
    /// use image_template::{Image, AlphaPixel};
    /// 
    /// let image: Image<u8> = Image::new_with_fill(AlphaPixel::black(), 5, 5);
    /// // First 2 pixels of the 3rd row:
    /// let slice = image.index_of_unchecked(0, 2)..image.index_of_unchecked(2, 2);
    /// let pixels = image.get_pixels().get(slice).unwrap();
    /// assert_eq!(pixels, &[AlphaPixel::black(); 2])
    /// ```
    pub fn index_of_unchecked(&self, x: usize, y: usize) -> usize {
        self.width*y + x
    }

    /// Get the index into the collection of pixels for a given coordinate.
    /// Returns `None` if the coordinate is not within the image's bounds.
    /// 
    /// ```
    /// use image_template::{Image, AlphaPixel};
    /// 
    /// let image: Image<u8> = Image::new_with_fill(AlphaPixel::black(), 5, 5);
    /// // In practice, use `image.pixel_at(0, 0)` instead.
    /// assert_eq!(image.get_pixels()[image.index_of(0, 0).unwrap()], AlphaPixel::black());
    /// assert!(image.index_of(10, 10).is_none());
    /// ```
    pub fn index_of(&self, x: usize, y: usize) -> Option<usize> {
        if self.contains(x, y) {
            Some(self.index_of_unchecked(x, y))
        } else {
            None
        }
    }

    /// Get a row of pixels as a slice
    /// 
    /// Returns `None` if `y >= image.get_height()`
    /// 
    /// ```
    /// use image_template::{Image, AlphaPixel};
    /// 
    /// let image: Image<u8> = Image::new_with_fill(AlphaPixel::black(), 5, 5);
    /// let second_row = image.row(1).unwrap();
    /// assert_eq!(second_row, &[AlphaPixel::black(); 5])
    /// ```
    pub fn row(&self, y: usize) -> Option<&[AlphaPixel<T>]> {
        // Last index may not be in the image, but this is okay as the range is exclusive.
        let range = self.index_of(0, y)?..self.index_of_unchecked(0, y+1);
        self.pixels.get(range)
    }

    /// Get a row of pixels as a mutable slice
    /// 
    /// Returns `None` if `y >= image.get_height()`
    /// 
    /// ```
    /// use image_template::{Image, AlphaPixel};
    /// 
    /// let mut image: Image<u8> = Image::new_with_fill(AlphaPixel::black(), 5, 5);
    /// let second_row = image.row_mut(1).unwrap();
    /// second_row.fill(AlphaPixel::red());
    /// assert_eq!(second_row, &[AlphaPixel::red(); 5])
    pub fn row_mut(&mut self, y: usize) -> Option<&mut [AlphaPixel<T>]> {
        let range = self.index_of(0, y)?..self.index_of(0, y+1)?;
        self.pixels.get_mut(range)
    }

    /// Get the pixel at a given coordinate.
    /// Returns `None` if the coordinate isn't in bounds.
    /// 
    /// ```
    /// use image_template::{Image, AlphaPixel};
    /// 
    /// let image: Image<u8> = Image::new_with_fill(AlphaPixel::black(), 5, 5);
    /// assert_eq!(image.pixel_at(0 , 0).unwrap(), AlphaPixel::black());
    /// ```
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<AlphaPixel<T>> {
        self.pixels.get(self.index_of(x, y)?).copied()
    }

    /// Get the pixel at a given coordinate.
    /// Returns `None` if the coordinate isn't in bounds.
    /// 
    /// ```
    /// use image_template::{Image, AlphaPixel};
    /// 
    /// let mut image: Image<u8> = Image::new_with_fill(AlphaPixel::black(), 5, 5);
    /// *image.pixel_at_mut(0, 0).unwrap() = AlphaPixel::red();
    /// assert_eq!(image.pixel_at(0, 0).unwrap(), AlphaPixel::red());
    /// ```
    pub fn pixel_at_mut(&mut self, x: usize, y: usize) -> Option<&mut AlphaPixel<T>> {
        let idx = self.index_of(x, y)?;
        self.pixels.get_mut(idx)
    }

    /// Check whether a coordinate is within the bounds of an image.
    /// 
    /// ```
    /// use image_template::{Image, AlphaPixel};
    /// 
    /// let image: Image<u8> = Image::new_with_fill(AlphaPixel::black(), 10, 5);
    /// assert!(image.contains(2, 2));
    /// assert!(image.contains(2, 4));
    /// assert!(!image.contains(10, 5));
    /// assert!(!image.contains(0, 50));
    /// ```
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Draw another image on top of this image at a coordinate. The subimage is cut off at the edges of this image.
    /// 
    /// `blend` is the method to combine the foreground and background. For most cases use [`BlendingMethod::Over`].
    /// 
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
}

impl<T: PixelChannel> AsRef<[u8]> for Image<T> {
    fn as_ref(&self) -> &[u8] {
        must_cast_slice(&self.pixels)
    }
}

#[cfg(feature = "image-crate")]
use {
    std::path::Path,
    std::fs::File,
    std::io::BufReader,
    image::{
        error::{ParameterError, ParameterErrorKind},
        save_buffer_with_format,
        GenericImageView,
        ImageFormat,
        Pixel,
        DynamicImage,
        ImageResult,
        ImageError
    }
};
#[cfg(feature = "image-crate")]
impl<T: PixelChannel> Image<T> {
    /// Save an image as a file.
    /// 
    /// Will fail if width or height cannot fit into a `u32`
    pub fn save<P: AsRef<Path>>(&self, path: P, format: ImageFormat) -> image::ImageResult<()> {
        let width: u32 = self.width.try_into()
            .map_err(|_| ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)))?;
        let height: u32 = self.height.try_into()
            .map_err(|_| ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)))?;

        save_buffer_with_format(path, self.as_ref(), width, height, AlphaPixel::<T>::color_type(), format)
    }

    pub fn load_from_memory<B: AsRef<[u8]>>(buffer: B, format: ImageFormat) -> ImageResult<Image<T>> {
        image::load_from_memory_with_format(buffer.as_ref(), format).map(|im| im.into())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P, format: ImageFormat) -> ImageResult<Image<T>> {
        let file = File::open(path)
            .map_err(ImageError::IoError)?;

        image::load(BufReader::new(file), format).map(|im| im.into())
    }
}

#[cfg(feature = "image-crate")]
impl<T: PixelChannel> From<DynamicImage> for Image<T> {
    fn from(value: DynamicImage) -> Self {
        let (width, height) = (value.width() as usize, value.height() as usize);
        
        let pixel_buf = match AlphaPixel::<T>::color_type() {
            image::ColorType::Rgba8 => {
                let buf = value.into_rgba8().into_raw();

                // Since T should be u8 when color type is Rgba8, this should be optimised away.
                let buf_generic = buf.iter().map(|p| T::from_u8(*p).unwrap()).collect();

                AlphaPixel::try_pixel_vec_from_channels(buf_generic).unwrap()
            },
            image::ColorType::Rgba16 => {
                let buf = value.into_rgba16().into_raw();
                
                // This should also be optimised away.
                let buf_generic = buf.iter().map(|p| T::from_u16(*p).unwrap()).collect();

                AlphaPixel::try_pixel_vec_from_channels(buf_generic).unwrap()
            },
            _ => unimplemented!(),
        };

        Self { pixels: pixel_buf, width, height }
    }
}

#[cfg(feature = "image-crate")]
impl<T> GenericImageView for Image<T>
where 
    T: PixelChannel,
    AlphaPixel<T>: Pixel
{
    type Pixel = AlphaPixel<T>;

    fn dimensions(&self) -> (u32, u32) {
        (self.get_width() as u32, self.get_height() as u32)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        Image::pixel_at(self, x as usize, y as usize).unwrap()
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

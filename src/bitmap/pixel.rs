use bytemuck::NoUninit;
use num_traits::{FromPrimitive, Num, NumCast};
use thiserror::Error;
use std::fmt::Debug;
use std::mem::ManuallyDrop;

#[derive(Debug)]
pub enum VecCastErrorKind {
    IncorrectCapacity,
    IncorrectLength,
    InvalidChannelValue
}

#[derive(Error)]
pub struct VecCastError<T>{
    pub original_vec: Vec<T>,
    pub kind: VecCastErrorKind
}   

impl<T> Debug for VecCastError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error casting Vec: {:?}", self.kind)
    }
}

pub trait PixelChannelBounds: PartialOrd + Copy {
    const MAX_PIXEL_VALUE: Self;
    const MIN_PIXEL_VALUE: Self;

    fn is_valid_channel_value(self) -> bool {
        Self::MIN_PIXEL_VALUE <= self && Self::MAX_PIXEL_VALUE >= self
    }
}

impl PixelChannelBounds for u8 {
    const MAX_PIXEL_VALUE: Self = Self::MAX;
    const MIN_PIXEL_VALUE: Self = Self::MIN;

    fn is_valid_channel_value(self) -> bool {
        true
    }
}

impl PixelChannelBounds for u16 {
    const MAX_PIXEL_VALUE: Self = Self::MAX;
    const MIN_PIXEL_VALUE: Self = Self::MIN;

    fn is_valid_channel_value(self) -> bool {
        true
    }
}

impl PixelChannelBounds for f32 {
    const MAX_PIXEL_VALUE: Self = 1.0;
    const MIN_PIXEL_VALUE: Self = 0.0;
}

// Requires Into<f32> for some float maths. TODO: Look into alternatives?
pub trait PixelChannel: Num + NumCast + FromPrimitive + PixelChannelBounds + Into<f32> + NoUninit {}

impl PixelChannel for u8 {}

impl PixelChannel for u16 {}

impl PixelChannel for f32 {}

#[macro_export]
macro_rules! rgba {
    ($r: literal, $g: literal, $b: literal, $a: literal) => {
        $crate::AlphaPixel { r: $r, g: $g, b: $b, a: $a }
    };

    ($r: expr, $g: expr, $b: expr, $a: expr) => {
        $crate::AlphaPixel { r: $r, g: $g, b: $b, a: $a }
    };
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
/// A RGBA pixel, generic over the channel type `T`.
/// 
/// The layout of this type will always be equal to `[T; 4]`.
pub struct AlphaPixel<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}

impl<T: PixelChannel> AlphaPixel<T> {
    /// `T: u8` rgba(255, 255, 255, 255)
    pub fn white() -> Self {
        Self { r: T::MAX_PIXEL_VALUE, g: T::MAX_PIXEL_VALUE, b: T::MAX_PIXEL_VALUE, a: T::MAX_PIXEL_VALUE  }
    }

    /// `T: u8` rgba(0, 0, 0, 0)
    pub fn black() -> Self {
        Self { r: T::zero(), g: T::zero(), b: T::zero(), a: T::MAX_PIXEL_VALUE }
    }

    /// `T: u8` rgba(255, 0, 0, 255)
    pub fn red() -> Self {
        Self { r: T::MAX_PIXEL_VALUE, g: T::zero(), b: T::zero(), a: T::MAX_PIXEL_VALUE }
    }

    /// `T: u8` rgba(0, 255, 0, 255)
    pub fn green() -> Self {
        Self { r: T::zero(), g: T::MAX_PIXEL_VALUE, b: T::zero(), a: T::MAX_PIXEL_VALUE }
    }

    /// `T: u8` rgba(0, 0, 255, 255)
    pub fn blue() -> Self {
        Self { r: T::zero(), g: T::zero(), b: T::MAX_PIXEL_VALUE, a: T::MAX_PIXEL_VALUE }
    }

    /// Get a value representing the luminosity of this pixel, by NTSC formula.
    pub fn luma(self) -> T {
        let float_pixel: AlphaPixel<f32> = self.as_float_pixel();
        let luma = 0.299 * float_pixel.r + 0.587 * float_pixel.g + 0.114 * float_pixel.b;
        T::from_f32(luma*(T::MAX_PIXEL_VALUE.into())).unwrap()
    }

    fn invert(&mut self) {
        self.r = T::MAX_PIXEL_VALUE - self.r;
        self.g = T::MAX_PIXEL_VALUE - self.g;
        self.b = T::MAX_PIXEL_VALUE - self.b;
    }

    pub fn as_float_pixel(&self) -> AlphaPixel<f32> {
        AlphaPixel {
            r: self.r.into() / T::MAX_PIXEL_VALUE.into(),
            g: self.g.into() / T::MAX_PIXEL_VALUE.into(),
            b: self.b.into() / T::MAX_PIXEL_VALUE.into(),
            a: self.a.into() / T::MAX_PIXEL_VALUE.into()
        }
    }

    /// Convert from `AlphaPixel<T>` to `AlphaPixel<U>`, by converting to a float pixel and multiplying by `U::MAX_PIXEL_VALUE`
    /// 
    /// # Example
    /// ```
    /// use image_template::AlphaPixel;
    /// 
    /// let pixel_u8: AlphaPixel<u8> = AlphaPixel::red();
    /// let pixel_u16: AlphaPixel<u16> = pixel_u8.as_different_channel();
    /// assert_eq!(pixel_u16, AlphaPixel::<u16>::red());
    /// ```
    pub fn as_different_channel<U: PixelChannel>(&self) -> AlphaPixel<U> {
        let float_pixel = self.as_float_pixel();
        AlphaPixel {
            r: U::from_f32(U::MAX_PIXEL_VALUE.into()*float_pixel.r).unwrap(),
            g: U::from_f32(U::MAX_PIXEL_VALUE.into()*float_pixel.g).unwrap(),
            b: U::from_f32(U::MAX_PIXEL_VALUE.into()*float_pixel.b).unwrap(),
            a: U::from_f32(U::MAX_PIXEL_VALUE.into()*float_pixel.a).unwrap()
        }
    }

    /// Get a hex string from a pixel.
    /// 
    /// # Example
    /// ```
    /// use image_template::AlphaPixel;
    /// 
    /// let pixel: AlphaPixel<u8> = AlphaPixel::red();
    /// assert_eq!(pixel.as_hex_string(), "ff0000ff");
    /// ```
    pub fn as_hex_string(&self) -> String {
        let u8_pixel: AlphaPixel<u8> = self.as_different_channel();
        format!("{:02x}{:02x}{:02x}{:02x}", u8_pixel.r, u8_pixel.g, u8_pixel.b, u8_pixel.a)
    }

    /// Get a slice of the pixel's channels.
    /// 
    /// # Example
    /// ```
    /// use image_template::AlphaPixel;
    /// let white = AlphaPixel::<u8>::white();
    /// assert_eq!(white.channels(), &[255, 255, 255, 255]);
    /// ```
    pub fn channels(&self) -> &[T] {
        let first_subpixel_ptr = self as *const AlphaPixel<T> as *const T;
        // Safety: The layout of `AlphaPixel<T>` is the same as [T; 4]
        unsafe { std::slice::from_raw_parts(first_subpixel_ptr, 4) }
    }

    /// Get a mutable slice of the pixel's channels.
    /// 
    /// # Example
    /// ```
    /// use image_template::AlphaPixel;
    /// let mut white = AlphaPixel::<u8>::white();
    /// white.channels_mut()[3] = 0;
    /// assert_eq!(white.channels(), &[255, 255, 255, 0]);
    /// ```
    pub fn channels_mut(&mut self) -> &mut [T] {
        let first_subpixel_ptr = self as *mut AlphaPixel<T> as *mut T;
        // Safety: The layout of `AlphaPixel<T>` is the same as [T; 4]
        unsafe { std::slice::from_raw_parts_mut(first_subpixel_ptr, 4) }
    }

    /// Convert from a slice of pixel components with a length of at least 4, to an `AlphaPixel` reference.
    /// 
    /// Note that if there are more than 4 values, they will be ignored.
    /// 
    /// # None
    /// Returns None if the slice doesn't have at least 4 components, or some of the components don't
    /// have valid values.
    /// 
    /// # Example
    /// ```
    /// use image_template::AlphaPixel;
    /// 
    /// let components = [255u8, 255, 255, 255];
    /// let pixel = *AlphaPixel::try_from_slice(&components).unwrap();
    /// assert_eq!(pixel, AlphaPixel::white());
    /// ```
    pub fn try_from_slice(slice: &[T]) -> Option<&Self> {
        if slice.len() >= 4 && slice.iter().take(4).all(|p| p.is_valid_channel_value()) {
            // Safety: the first 4 `T`s of the slice have valid values for a pixel channel.
            // `AlphaPixel<T>` has the same layout as [T; 4], and therefore the same layout
            // as &[T] with a length of >= 4.
            Some(unsafe { &*(slice.as_ptr() as *const AlphaPixel<T>) })
        } else {
            None
        }
    }

    /// Convert from a mutable slice of pixel components with a length of at least 4,
    /// to a mutable `AlphaPixel` reference.
    /// 
    /// Note that if there are more than 4 values, they will be ignored.
    /// 
    /// # None
    /// Returns None if the slice doesn't have at least 4 components, or some of the components don't
    /// have valid values.
    /// 
    /// # Example
    /// ```
    /// use image_template::AlphaPixel;
    /// 
    /// let mut components = [255u8, 255, 255, 255];
    /// let mut pixel = AlphaPixel::try_from_slice_mut(&mut components).unwrap();
    /// pixel.r = 0;
    /// assert_eq!(components, [0, 255, 255, 255]);
    /// ```
    pub fn try_from_slice_mut(slice: &mut [T]) -> Option<&mut Self> {
        if slice.len() >= 4 && slice.iter().take(4).all(|p| p.is_valid_channel_value()) {
            // Safety: the first 4 `T`s of the slice have valid values for a pixel channel.
            // `AlphaPixel<T>` has the same layout as [T; 4], and therefore the same layout
            // as &[T] with a length of >= 4.
            Some(unsafe { &mut *(slice.as_mut_ptr() as *mut AlphaPixel<T>) })
        } else {
            None
        }
    }

    /// Convert from a slice of components to a slice of AlphaPixels.
    /// 
    /// # None
    /// Returns None if any of the components are invalid.
    /// 
    /// # Example
    /// ```
    /// use image_template::AlphaPixel;
    /// 
    /// let components = [255u8, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 100];
    /// let pixel_slice = AlphaPixel::try_pixel_slice_from_channels(&components).unwrap();
    /// assert_eq!(pixel_slice, [AlphaPixel::white(), AlphaPixel::black(), AlphaPixel::white()]);
    /// 
    /// let empty: [u8; 0] = [];
    /// let empty_pixel_slice = AlphaPixel::try_pixel_slice_from_channels(&empty).unwrap();
    /// assert_eq!(empty_pixel_slice, []);
    /// ```
    pub fn try_pixel_slice_from_channels(channel_slice: &[T]) -> Option<&[AlphaPixel<T>]> {
        if channel_slice.iter().any(|p| !p.is_valid_channel_value()) {
            return None
        }

        let new_slice_len = channel_slice.len() / 4;
        let new_start_ptr = channel_slice.as_ptr() as *const AlphaPixel<T>;
        // Safety: pointer is aligned as `AlphaPixel<T>` has an alignment of T.
        // `AlphaPixel<T>` has the same layout as [T; 4]. This is the same as casting &[T] to &[[T; 4]].
        // The new length is valid as `channel_slice` contains `new_slice_len` amount of [T; 4].
        // All `T` are checked to have valid values. 
        Some(unsafe { std::slice::from_raw_parts(new_start_ptr, new_slice_len) })
    }

    /// Convert from a mutable slice of components to a mutable slice of AlphaPixels.
    /// 
    /// # None
    /// Returns None if any of the components are invalid.
    /// 
    /// # Example
    /// ```
    /// use image_template::AlphaPixel;
    /// 
    /// let mut components = [255u8, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 100];
    /// let mut pixel_slice = AlphaPixel::try_pixel_slice_from_channels_mut(&mut components).unwrap();
    /// pixel_slice[0].r = 0;
    /// pixel_slice[0].g = 0;
    /// assert_eq!(components, [0, 0, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 100]);
    /// ```
    pub fn try_pixel_slice_from_channels_mut(channel_slice: &mut [T]) -> Option<&mut [AlphaPixel<T>]> {
        if channel_slice.iter().any(|p| !p.is_valid_channel_value()) {
            return None
        }

        let new_slice_len = channel_slice.len() / 4;
        let new_start_ptr = channel_slice.as_mut_ptr() as *mut AlphaPixel<T>;
        // Safety: pointer is aligned as `AlphaPixel<T>` has an alignment of T.
        // `AlphaPixel<T>` has the same layout as [T; 4]. This is the same as casting &[T] to &[[T; 4]].
        // The new length is valid as `channel_slice` contains `new_slice_len` amount of [T; 4].
        // All `T` are checked to have valid values. 
        Some(unsafe { std::slice::from_raw_parts_mut(new_start_ptr, new_slice_len) })
    }

    /// Convert from a `Vec` of components to a `Vec` of `AlphaPixel`s.
    /// 
    /// # Error
    /// Returns `Err` if:
    /// - Any of the components are invalid.
    /// - The length of the Vec (in `T`s) is not a multiple of 4.
    /// - The capacity of the Vec (in `T`s) is not a multiple of 4.
    /// 
    /// # Example
    /// ```
    /// use image_template::AlphaPixel;
    /// 
    /// let component_vec = vec![255u8, 255, 255, 255, 0, 0, 0, 255];
    /// let pixel_vec = AlphaPixel::try_pixel_vec_from_channels(component_vec).unwrap();
    /// assert_eq!(pixel_vec, [AlphaPixel::white(), AlphaPixel::black()]);
    /// 
    /// let mut invalid_component_vec = vec![255u8, 255, 255, 255, 0, 0, 0, 255, 10];
    /// assert!(AlphaPixel::try_pixel_vec_from_channels(invalid_component_vec).is_err());
    /// ```
    pub fn try_pixel_vec_from_channels(channel_vec: Vec<T>) -> Result<Vec<AlphaPixel<T>>, VecCastError<T>> {
        if channel_vec.iter().any(|p| !p.is_valid_channel_value()) {
            return Err(VecCastError { original_vec: channel_vec, kind: VecCastErrorKind::InvalidChannelValue })
        }

        // Inspired by https://docs.rs/bytemuck/1.16.1/bytemuck/allocation/fn.try_cast_vec.html
        if channel_vec.len() % 4 == 0 {
            if channel_vec.capacity() % 4 == 0 {
                let new_length = channel_vec.len() / 4;
                let new_cap = channel_vec.capacity() / 4;

                let mut manual_drop_vec = ManuallyDrop::new(channel_vec);
                let ptr = manual_drop_vec.as_mut_ptr() as *mut AlphaPixel<T>;
                // Safety: AlphaPixel<T> has same alignment as T.
                // There are the same amount of bytes in the length and capacity.
                // All T are valid channel values, and there are perfect chunks of AlphaPixel<T>
                Ok(unsafe { Vec::from_raw_parts(ptr, new_length, new_cap) })
            } else {
                Err(VecCastError { original_vec: channel_vec, kind: VecCastErrorKind::IncorrectCapacity })
            }
        } else {
            Err(VecCastError { original_vec: channel_vec, kind: VecCastErrorKind::IncorrectLength })
        }
    }
}

/// Safety: `AlphaPixel` has no padding and all T: PixelChannel are NoUninit
unsafe impl<T: PixelChannel + 'static> NoUninit for AlphaPixel<T> {}

impl<T: Debug> Debug for AlphaPixel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("rgba").field(&self.r).field(&self.g).field(&self.b).field(&self.a).finish()
    }
}

impl<T: PixelChannel> Default for AlphaPixel<T> {
    fn default() -> Self {
        Self { r: T::zero(), g: T::zero(), b: T::zero(), a: T::zero() }
    }
}

#[cfg(feature = "image-crate")]
use {image::{ColorType, Primitive, Pixel}, std::mem::size_of};
#[cfg(feature = "image-crate")]
impl<T: PixelChannel> AlphaPixel<T> {
    /// Get the `image::ColorType` for this pixel
    pub const fn color_type() -> ColorType {
        if size_of::<T>() == 1 {
            ColorType::Rgba8
        } else if size_of::<T>() == 2 {
            ColorType::Rgba16
        } else if size_of::<T>() == 4 {
            ColorType::Rgba32F
        } else {
            unreachable!()
        }
    }
}

#[cfg(feature = "image-crate")]
impl<T> Pixel for AlphaPixel<T>
where
    T: Primitive + PixelChannel,
{
    type Subpixel = T;
    const CHANNEL_COUNT: u8 = 4;
    const COLOR_MODEL: &'static str = "RGBA";

    fn channels(&self) -> &[Self::Subpixel] {
        Self::channels(self)
    }

    fn channels_mut(&mut self) -> &mut [Self::Subpixel] {
        Self::channels_mut(self)
    }

    fn channels4(&self) -> (Self::Subpixel, Self::Subpixel, Self::Subpixel, Self::Subpixel) {
        (self.r, self.g, self.b, self.a)
    }

    fn from_channels(a: Self::Subpixel, b: Self::Subpixel, c: Self::Subpixel, d: Self::Subpixel) -> Self {
        Self { r: a, g: b, b: c, a: d }
    }

    fn from_slice(slice: &[Self::Subpixel]) -> &Self {
        Self::try_from_slice(slice).unwrap()
    }

    fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self {
        Self::try_from_slice_mut(slice).unwrap()
    }

    fn to_rgb(&self) -> image::Rgb<Self::Subpixel> {
        image::Rgb([self.r, self.g, self.b])
    }

    fn to_rgba(&self) -> image::Rgba<Self::Subpixel> {
        image::Rgba([self.r, self.g, self.b, self.a])
    }

    fn to_luma(&self) -> image::Luma<Self::Subpixel> {
        image::Luma([self.luma()])
    }

    fn to_luma_alpha(&self) -> image::LumaA<Self::Subpixel> {
        image::LumaA([self.luma(), self.a])
    }

    fn map<F>(&self, mut f: F) -> Self
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel {
        Self { r: f(self.r), g: f(self.g), b: f(self.b), a: f(self.a) }
    }

    fn apply<F>(&mut self, mut f: F)
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel {
        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);
        self.a = f(self.a);
    }

    fn map_with_alpha<F, G>(&self, mut f: F, mut g: G) -> Self
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel {
            Self { r: f(self.r), g: f(self.g), b: f(self.b), a: g(self.a) }
    }

    fn apply_with_alpha<F, G>(&mut self, mut f: F, mut g: G)
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel {
        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);
        self.a = g(self.a);
    }

    fn map2<F>(&self, other: &Self, mut f: F) -> Self
    where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel {
        Self {
            r: f(self.r, other.r),
            g: f(self.g, other.g),
            b: f(self.b, other.b),
            a: f(self.a, other.a)
        }
    }

    fn apply2<F>(&mut self, other: &Self, mut f: F)
    where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel {
        self.r = f(self.r, other.r);
        self.g = f(self.g, other.g);
        self.b = f(self.b, other.b);
        self.a = f(self.a, other.a);
    }

    fn invert(&mut self) {
        Self::invert(self)
    }

    fn blend(&mut self, other: &Self) {
        *self = crate::bitmap::blending::BlendingMethod::Over.blend(*self, *other)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::{align_of, size_of, align_of_val, size_of_val};
    use super::*;

    #[test]
    fn pixel_no_padding() {
        assert_eq!(size_of::<AlphaPixel<u8>>(), 4);
        assert_eq!(size_of::<AlphaPixel<u16>>(), 8);
        assert_eq!(size_of::<AlphaPixel<f32>>(), 16);
    }

    #[test]
    fn pixel_alignment() {
        assert_eq!(align_of::<AlphaPixel<u8>>(), 1);
        assert_eq!(align_of::<AlphaPixel<u16>>(), 2);
        assert_eq!(align_of::<AlphaPixel<f32>>(), 4);
    }

    #[test]
    fn pixel_float_conversion() {
        let max_pixel: AlphaPixel<u8> = rgba!(255, 255, 255, 255);
        let max_float_pixel: AlphaPixel<f32> = max_pixel.as_float_pixel();
        assert_eq!(max_float_pixel, rgba!(1.0, 1.0, 1.0, 1.0));

        let min_pixel: AlphaPixel<u8> = rgba!(0, 0, 0, 0);
        let min_float_pixel: AlphaPixel<f32> = min_pixel.as_float_pixel();
        assert_eq!(min_float_pixel, rgba!(0.0, 0.0, 0.0, 0.0));

        let fraction_pixel: AlphaPixel<u8> = rgba!(102, 204, 51, 0);
        let fraction_float_pixel: AlphaPixel<f32> = fraction_pixel.as_float_pixel();
        assert_eq!(fraction_float_pixel, rgba!(0.4, 0.8, 0.2, 0.0));
    }

    #[test]
    fn debug() {
        let pixel1 = rgba!(255u8, 255, 255, 255);
        assert_eq!("rgba(255, 255, 255, 255)", format!("{:?}", pixel1));

        let pixel2 = rgba!(1000u16, 10, 1, 0);
        assert_eq!("rgba(1000, 10, 1, 0)", format!("{:?}", pixel2));
    }

    #[test]
    fn create_pixel_macro() {
        assert_eq!(rgba!(0u8, 0, 0, 255), AlphaPixel { r: 0, g: 0, b: 0, a: 255 });
        assert_eq!(rgba!(1000u16, 2000, 0, 100), AlphaPixel { r: 1000, g: 2000, b: 0, a: 100 });
    }

    
    #[test]
    #[cfg(feature = "image-crate")]
    fn color_type() {
        assert_eq!(AlphaPixel::<u8>::color_type(), image::ColorType::Rgba8);
        assert_eq!(AlphaPixel::<u16>::color_type(), image::ColorType::Rgba16);
        assert_eq!(AlphaPixel::<f32>::color_type(), image::ColorType::Rgba32F);
    }

    #[test]
    #[cfg(feature = "image-crate")]
    fn test_channels() {
        // Test that AlphaPixel<T: PixelChannel> and [T; 4] have the same size and alignment.
        // Running miri on this test should catch if `.channels` or `.channels_mut` are unsafe.

        // u8
        let mut pixel: AlphaPixel<u8> = AlphaPixel::black();
        let channels = pixel.channels();

        assert_eq!(align_of_val(channels), align_of::<AlphaPixel<u8>>());
        assert_eq!(size_of_val(channels), size_of::<AlphaPixel<u8>>());
        assert_eq!(channels.len(), 4);

        let channels_mut = pixel.channels_mut();
        assert_eq!(align_of_val(channels_mut), align_of::<AlphaPixel<u8>>());
        assert_eq!(size_of_val(channels_mut), size_of::<AlphaPixel<u8>>());
        assert_eq!(channels_mut.len(), 4);

        // u16
        let mut pixel: AlphaPixel<u16> = AlphaPixel::black();
        let channels = pixel.channels();

        assert_eq!(align_of_val(channels), align_of::<AlphaPixel<u16>>());
        assert_eq!(size_of_val(channels), size_of::<AlphaPixel<u16>>());
        assert_eq!(channels.len(), 4);

        let channels_mut = pixel.channels_mut();
        assert_eq!(align_of_val(channels_mut), align_of::<AlphaPixel<u16>>());
        assert_eq!(size_of_val(channels_mut), size_of::<AlphaPixel<u16>>());
        assert_eq!(channels_mut.len(), 4);

        // f32
        let mut pixel: AlphaPixel<f32> = AlphaPixel::black();
        let channels = pixel.channels();

        assert_eq!(align_of_val(channels), align_of::<AlphaPixel<f32>>());
        assert_eq!(size_of_val(channels), size_of::<AlphaPixel<f32>>());
        assert_eq!(channels.len(), 4);

        let channels_mut = pixel.channels_mut();
        assert_eq!(align_of_val(channels_mut), align_of::<AlphaPixel<f32>>());
        assert_eq!(size_of_val(channels_mut), size_of::<AlphaPixel<f32>>());
        assert_eq!(channels_mut.len(), 4);
    }
}

use bytemuck::NoUninit;
use num_traits::{Bounded, FromPrimitive, Num, NumCast};
use std::fmt::Debug;
use std::mem::ManuallyDrop;

// Requires Into<f32> for some float maths. TODO: Look into alternatives?
pub trait PixelChannel: Copy + Num + NumCast + FromPrimitive + Bounded + Into<f32> + NoUninit + PartialOrd {}

impl PixelChannel for u8 {}
impl PixelChannel for u16 {}

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
    pub fn is_valid_channel_value(value: T) -> bool {
        T::min_value() <= value && T::max_value() >= value
    }

    pub fn white() -> Self {
        Self { r: T::max_value(), g: T::max_value(), b: T::max_value(), a: T::max_value()  }
    }

    pub fn black() -> Self {
        Self { r: T::zero(), g: T::zero(), b: T::zero(), a: T::max_value() }
    }

    pub fn red() -> Self {
        Self { r: T::max_value(), g: T::zero(), b: T::zero(), a: T::max_value() }
    }

    pub fn green() -> Self {
        Self { r: T::zero(), g: T::max_value(), b: T::zero(), a: T::max_value() }
    }

    pub fn blue() -> Self {
        Self { r: T::zero(), g: T::zero(), b: T::max_value(), a: T::max_value() }
    }

    pub fn luma(self) -> T {
        let float_pixel: AlphaPixel<f32> = self.into();
        let luma = 0.299 * float_pixel.r + 0.587 * float_pixel.g + 0.114 * float_pixel.b;
        T::from_f32(luma*255.0).unwrap()
    }

    pub fn channels(&self) -> &[T] {
        let first_subpixel_ptr = self as *const AlphaPixel<T> as *const T;
        // Safety: The layout of `AlphaPixel<T>` is the same as [T; 4]
        unsafe { std::slice::from_raw_parts(first_subpixel_ptr, 4) }
    }

    pub fn channels_mut(&mut self) -> &mut [T] {
        let first_subpixel_ptr = self as *mut AlphaPixel<T> as *mut T;
        // Safety: The layout of `AlphaPixel<T>` is the same as [T; 4]
        unsafe { std::slice::from_raw_parts_mut(first_subpixel_ptr, 4) }
    }

    pub fn try_from_slice(slice: &[T]) -> Option<&Self> {
        if slice.len() >= 4 && slice.iter().take(4).all(|p| AlphaPixel::is_valid_channel_value(*p)) {
            // Safety: the first 4 `T`s of the slice have valid values for a pixel channel.
            // `AlphaPixel<T>` has the same layout as [T; 4], and therefore the same layout
            // as &[T] with a length of >= 4.
            Some(unsafe { &*(slice.as_ptr() as *const AlphaPixel<T>) })
        } else {
            None
        }
    }

    pub fn try_from_slice_mut(slice: &mut [T]) -> Option<&mut Self> {
        if slice.len() >= 4 && slice.iter().take(4).all(|p| AlphaPixel::is_valid_channel_value(*p)) {
            // Safety: the first 4 `T`s of the slice have valid values for a pixel channel.
            // `AlphaPixel<T>` has the same layout as [T; 4], and therefore the same layout
            // as &[T] with a length of >= 4.
            Some(unsafe { &mut *(slice.as_mut_ptr() as *mut AlphaPixel<T>) })
        } else {
            None
        }
    }

    pub fn try_pixel_slice_from_channels(channel_slice: &[T]) -> Option<&[AlphaPixel<T>]> {
        if channel_slice.iter().any(|p| !AlphaPixel::is_valid_channel_value(*p)) {
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

    pub fn try_pixel_slice_from_channels_mut(channel_slice: &mut [T]) -> Option<&mut [AlphaPixel<T>]> {
        if channel_slice.iter().any(|p| !AlphaPixel::is_valid_channel_value(*p)) {
            return None
        }

        let new_slice_len = channel_slice.len() / 4;
        let new_start_ptr = channel_slice.as_ptr() as *mut AlphaPixel<T>;
        // Safety: pointer is aligned as `AlphaPixel<T>` has an alignment of T.
        // `AlphaPixel<T>` has the same layout as [T; 4]. This is the same as casting &[T] to &[[T; 4]].
        // The new length is valid as `channel_slice` contains `new_slice_len` amount of [T; 4].
        // All `T` are checked to have valid values. 
        Some(unsafe { std::slice::from_raw_parts_mut(new_start_ptr, new_slice_len) })
    }

    pub fn try_pixel_vec_from_channels(channel_vec: Vec<T>) -> Option<Vec<AlphaPixel<T>>> {
        if channel_vec.iter().any(|p| !AlphaPixel::is_valid_channel_value(*p)) {
            return None
        }

        // Inspired by https://docs.rs/bytemuck/1.16.1/bytemuck/allocation/fn.try_cast_vec.html
        if channel_vec.len() % 4 == 0 && channel_vec.capacity() % 4 == 0 {
            let new_length = channel_vec.len() / 4;
            let new_cap = channel_vec.capacity() / 4;

            let mut manual_drop_vec = ManuallyDrop::new(channel_vec);
            let ptr = manual_drop_vec.as_mut_ptr() as *mut AlphaPixel<T>;
            // Safety: AlphaPixel<T> has same alignment as T.
            // There are the same amount of bytes in the length and capacity.
            // All T are valid channel values, and there are perfect chunks of AlphaPixel<T>
            Some(unsafe { Vec::from_raw_parts(ptr, new_length, new_cap) })
        } else {
            None
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

/// Convert an `AlphaPixel<T: PixelChannel>` to a `AlphaPixel<f32>`, where each component is in the range 0-1
impl<T: PixelChannel> From<AlphaPixel<T>> for AlphaPixel<f32> {
    fn from(value: AlphaPixel<T>) -> Self {
        Self {
            r: value.r.into() / T::max_value().into(),
            g: value.g.into() / T::max_value().into(),
            b: value.b.into() / T::max_value().into(),
            a: value.a.into() / T::max_value().into()
        }
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

// TODO: Move many of these methods to `AlphaPixel<T>`

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
        self.r = T::max_value() - self.r;
        self.g = T::max_value() - self.g;
        self.b = T::max_value() - self.b;
    }

    fn blend(&mut self, other: &Self) {
        *self = crate::bitmap::blending::BlendingMethod::Over.blend(*self, *other)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::{align_of, size_of};
    use super::*;

    #[test]
    fn pixel_no_padding() {
        assert_eq!(size_of::<AlphaPixel<u8>>(), 4);
        assert_eq!(size_of::<AlphaPixel<u16>>(), 8);
    }

    #[test]
    fn pixel_alignment() {
        assert_eq!(align_of::<AlphaPixel<u8>>(), 1);
        assert_eq!(align_of::<AlphaPixel<u16>>(), 2);
    }

    #[test]
    fn pixel_float_conversion() {
        let max_pixel: AlphaPixel<u8> = rgba!(255, 255, 255, 255);
        let max_float_pixel: AlphaPixel<f32> = max_pixel.into();
        assert_eq!(max_float_pixel, rgba!(1.0, 1.0, 1.0, 1.0));

        let min_pixel: AlphaPixel<u8> = rgba!(0, 0, 0, 0);
        let min_float_pixel: AlphaPixel<f32> = min_pixel.into();
        assert_eq!(min_float_pixel, rgba!(0.0, 0.0, 0.0, 0.0));

        let fraction_pixel: AlphaPixel<u8> = rgba!(102, 204, 51, 0);
        let fraction_float_pixel: AlphaPixel<f32> = fraction_pixel.into();
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
        assert_eq!(rgba!(0, 0, 0, 255), AlphaPixel { r: 0, g: 0, b: 0, a: 255 });
        assert_eq!(rgba!(1000, 2000, 0, 100), AlphaPixel { r: 1000, g: 2000, b: 0, a: 100 });
    }

    
    #[test]
    #[cfg(feature = "image-crate")]
    fn color_type() {
        assert_eq!(AlphaPixel::<u8>::color_type(), image::ColorType::Rgba8);
        assert_eq!(AlphaPixel::<u16>::color_type(), image::ColorType::Rgba16);
    }
}

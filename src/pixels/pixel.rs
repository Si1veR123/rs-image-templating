use bytemuck::NoUninit;
use num::{Bounded, FromPrimitive, Num, NumCast, Zero};
use image::ExtendedColorType;
use std::{fmt::Debug, mem::size_of};

// Requires Into<f32> for some float maths. TODO: Look into alternatives?
pub trait PixelChannel: Copy + Num + NumCast + FromPrimitive + Bounded + Into<f32> + NoUninit {}

impl PixelChannel for u8 {}
impl PixelChannel for u16 {}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct AlphaPixel<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}

impl<T: PixelChannel> AlphaPixel<T> {
    pub const fn color_type() -> ExtendedColorType {
        if size_of::<T>() == 1 {
            ExtendedColorType::Rgba8
        } else if size_of::<T>() == 2 {
            ExtendedColorType::Rgba16
        } else if size_of::<T>() == 4 {
            ExtendedColorType::Rgba32F
        } else {
            unreachable!()
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

/// Convert an AlphaPixel<T: PixelChannel> to a AlphaPixel<f32>, where each component is in the range 0-1
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

#[cfg(test)]
mod tests {
    use std::mem::align_of;
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
}

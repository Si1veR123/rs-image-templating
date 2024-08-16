use bytemuck::NoUninit;
use num::{Bounded, FromPrimitive, Num, NumCast};
use image::ColorType;
use std::{fmt::Debug, mem::size_of};

// Requires Into<f32> for some float maths. TODO: Look into alternatives?
pub trait PixelChannel: Copy + Num + NumCast + FromPrimitive + Bounded + Into<f32> + NoUninit {}

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
pub struct AlphaPixel<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}

impl<T: PixelChannel> AlphaPixel<T> {
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
}

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
    fn color_type() {
        assert_eq!(AlphaPixel::<u8>::color_type(), image::ColorType::Rgba8);
        assert_eq!(AlphaPixel::<u16>::color_type(), image::ColorType::Rgba16);
    }
}

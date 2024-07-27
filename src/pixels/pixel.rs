use num::{Bounded, FromPrimitive, Num, NumCast};
use std::fmt::Debug;

// Requires Into<f32> for some float maths. TODO: Look into alternatives?
pub trait PixelChannel: Copy + Num + NumCast + FromPrimitive + Bounded + Into<f32> {}

impl PixelChannel for u8 {}
impl PixelChannel for u16 {}

#[derive(Copy, Clone)]
pub struct AlphaPixel<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}

impl<T: Debug> Debug for AlphaPixel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.r).field(&self.g).field(&self.b).field(&self.a).finish()
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

pub fn over_operator<T: PixelChannel>(pixel1: &AlphaPixel<T>, pixel2: &AlphaPixel<T>) -> AlphaPixel<T> {
    let float_pixel1: AlphaPixel<f32> = pixel1.clone().into();
    let float_pixel2: AlphaPixel<f32> = pixel2.clone().into();

    let second_alpha_component = float_pixel2.a*(1.0-float_pixel1.a);
    let new_alpha = float_pixel1.a + second_alpha_component;

    let new_color_r = (float_pixel1.r*float_pixel1.a + float_pixel2.r*float_pixel2.a*second_alpha_component)/new_alpha;
    let new_color_g = (float_pixel1.g*float_pixel1.a + float_pixel2.g*float_pixel2.a*second_alpha_component)/new_alpha;
    let new_color_b = (float_pixel1.b*float_pixel1.a + float_pixel2.b*float_pixel2.a*second_alpha_component)/new_alpha;

    AlphaPixel {
        r: T::from_f32(new_color_r*T::max_value().into()).unwrap(),
        g: T::from_f32(new_color_g*T::max_value().into()).unwrap(),
        b: T::from_f32(new_color_b*T::max_value().into()).unwrap(),
        a: T::from_f32(new_alpha*T::max_value().into()).unwrap()
    }
}

use super::pixel::{AlphaPixel, PixelChannel};

pub enum BlendingMethod<'a, T: PixelChannel> {
    Replace,
    OverOperator,
    Custom(&'a dyn Fn(AlphaPixel<T>, AlphaPixel<T>) -> AlphaPixel<T>)
}

impl<'a, T: PixelChannel> BlendingMethod<'a, T> {
    /// `pixel2` is the foreground
    pub fn blend(&self, pixel1: AlphaPixel<T>, pixel2: AlphaPixel<T>) -> AlphaPixel<T> {
        match self {
            BlendingMethod::Replace => pixel2,
            BlendingMethod::OverOperator => over_operator(pixel2, pixel1),
            BlendingMethod::Custom(f) => f(pixel1, pixel2),
        }
    }
}

pub fn over_operator<T: PixelChannel>(pixel1: AlphaPixel<T>, pixel2: AlphaPixel<T>) -> AlphaPixel<T> {
    let float_pixel1: AlphaPixel<f32> = pixel1.into();
    let float_pixel2: AlphaPixel<f32> = pixel2.into();

    let second_alpha_component = float_pixel2.a*(1.0-float_pixel1.a);
    let new_alpha = float_pixel1.a + second_alpha_component;

    if new_alpha == 0.0 {
        return AlphaPixel::default()
    }

    let new_color_r = (float_pixel1.r*float_pixel1.a + float_pixel2.r*second_alpha_component)/new_alpha;
    let new_color_g = (float_pixel1.g*float_pixel1.a + float_pixel2.g*second_alpha_component)/new_alpha;
    let new_color_b = (float_pixel1.b*float_pixel1.a + float_pixel2.b*second_alpha_component)/new_alpha;

    AlphaPixel {
        r: T::from_f32(new_color_r*T::max_value().into()).unwrap(),
        g: T::from_f32(new_color_g*T::max_value().into()).unwrap(),
        b: T::from_f32(new_color_b*T::max_value().into()).unwrap(),
        a: T::from_f32(new_alpha*T::max_value().into()).unwrap()
    }
}

use crate::{AlphaPixel, PixelChannel};

pub enum BlendingMethod<'a, T: PixelChannel> {
    Replace,
    Over,
    Custom(&'a dyn Fn(AlphaPixel<T>, AlphaPixel<T>) -> AlphaPixel<T>)
}

impl<'a, T: PixelChannel> BlendingMethod<'a, T> {
    /// `pixel2` is the foreground
    pub fn blend(&self, pixel1: AlphaPixel<T>, pixel2: AlphaPixel<T>) -> AlphaPixel<T> {
        match self {
            BlendingMethod::Replace => pixel2,
            BlendingMethod::Over => over_operator(pixel2, pixel1),
            BlendingMethod::Custom(f) => f(pixel1, pixel2),
        }
    }
}

/// [Alpha Compositing](https://en.wikipedia.org/wiki/Alpha_compositing)
fn over_operator<T: PixelChannel>(pixel1: AlphaPixel<T>, pixel2: AlphaPixel<T>) -> AlphaPixel<T> {
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

#[cfg(test)]
mod tests {
    use crate::rgba;
    use super::*;

    #[test]
    fn blend_over_u8() {
        let cases = &[
            (rgba!(0u8, 0, 0, 0), rgba!(0, 0, 0, 0), rgba!(0, 0, 0, 0)),
            (rgba!(255u8, 255, 255, 255), rgba!(0, 0, 0, 0), rgba!(255, 255, 255, 255)),
            (rgba!(255u8, 255, 255, 255), rgba!(100, 100, 100, 0), rgba!(255, 255, 255, 255)),
            (rgba!(255u8, 255, 255, 0), rgba!(100, 100, 100, 255), rgba!(100, 100, 100, 255)),
            (rgba!(255u8, 255, 255, 0), rgba!(0, 0, 0, 255), rgba!(0, 0, 0, 255)),
            (rgba!(100u8, 0, 0, 255), rgba!(0, 50, 100, 255), rgba!(0, 50, 100, 255)),
            (rgba!(255u8, 255, 255, 255), rgba!(0, 50, 100, 25), rgba!(230, 234, 239, 255)),
            (rgba!(0u8, 0, 0, 255), rgba!(0, 50, 100, 204), rgba!(0, 40, 80, 255)),
            (rgba!(100u8, 0, 0, 255), rgba!(0, 50, 100, 102), rgba!(60, 20, 40, 255)),
            (rgba!(100u8, 55, 231, 102), rgba!(0, 50, 100, 255), rgba!(0, 50, 100, 255)),
            (rgba!(100u8, 55, 231, 102), rgba!(0, 50, 100, 34), rgba!(72, 53, 194, 122)),
            (rgba!(255u8, 55, 2, 102), rgba!(0, 50, 200, 254), rgba!(0, 50, 199, 254)),
            (rgba!(0u8, 0, 0, 53), rgba!(0, 0, 0, 212), rgba!(0, 0, 0, 220)),
            (rgba!(255u8, 255, 255, 200), rgba!(255, 255, 255, 145), rgba!(255, 255, 255, 231))
        ];

        for case in cases {
            assert_eq!(BlendingMethod::Over.blend(case.0, case.1), case.2);
        }
    }

    #[test]
    fn blend_replace_u8() {
        let cases = &[
            (rgba!(0u8, 0, 0, 0), rgba!(0, 0, 0, 0), rgba!(0, 0, 0, 0)),
            (rgba!(255u8, 255, 255, 255), rgba!(255, 255, 255, 255), rgba!(255, 255, 255, 255)),
            (rgba!(255u8, 255, 255, 255), rgba!(100, 0, 0, 0), rgba!(100, 0, 0, 0)),
            (rgba!(124u8, 43, 87, 0), rgba!(0, 0, 0, 0), rgba!(0, 0, 0, 0))
        ];

        for case in cases {
            assert_eq!(BlendingMethod::Replace.blend(case.0, case.1), case.2);
        }
    }
}


use crate::{Filter, AlphaPixel, PixelChannel};

pub struct BrightnessFilter {
    pub multiplier: f32
}

impl<T: PixelChannel> Filter<T> for BrightnessFilter {
    fn filter_pixel(&self, pixel: AlphaPixel<T>) -> AlphaPixel<T> {
        fn bounded_multiply(min: f32, max: f32, lhs: f32, rhs: f32) -> f32 {
            (lhs*rhs).min(max).max(min)
        }

        AlphaPixel {
            r: T::from_f32(bounded_multiply(0.0, 255.0, pixel.r.into(), self.multiplier)).unwrap(),
            g: T::from_f32(bounded_multiply(0.0, 255.0, pixel.g.into(), self.multiplier)).unwrap(),
            b: T::from_f32(bounded_multiply(0.0, 255.0, pixel.b.into(), self.multiplier)).unwrap(),
            a: pixel.a
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::layers::shapes::RectangleLayer;
    use crate::{rgba, Layer, Rect};

    use super::*;

    #[test]
    fn brightness() {
        let brightness_filter = Box::new(BrightnessFilter { multiplier: 2.0 });
        let rectangle: RectangleLayer<u8> = RectangleLayer {
            fill: rgba!(100, 100, 100, 255),
            rect: Rect { x: 0, y: 0, width: 100, height: 100 },
            filters: vec![brightness_filter]
        };
        assert_eq!(rectangle.unfiltered_pixel_at(50, 50).unwrap(), rgba!(100, 100, 100, 255));
        assert_eq!(rectangle.filtered_pixel_at(50, 50).unwrap(), rgba!(200, 200, 200, 255));
    }
}

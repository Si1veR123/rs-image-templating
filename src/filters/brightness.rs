use crate::{Filter, AlphaPixel, PixelChannel};

pub struct BrightnessFilter {
    pub multiplier: f32
}

impl<T: PixelChannel> Filter<T> for BrightnessFilter {
    fn filter_pixel(&self, pixel: AlphaPixel<T>) -> AlphaPixel<T> {
        let maximum = T::max_pixel_value().into();
        let minimum = T::min_pixel_value().into();

        AlphaPixel {
            r: T::from_f32((pixel.r.into() * self.multiplier).min(maximum).max(minimum)).unwrap(),
            g: T::from_f32((pixel.g.into() * self.multiplier).min(maximum).max(minimum)).unwrap(),
            b: T::from_f32((pixel.b.into() * self.multiplier).min(maximum).max(minimum)).unwrap(),
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
            fill: rgba!(100, 100, 200, 255),
            rect: Rect { x: 0, y: 0, width: 100, height: 100 },
            filters: vec![brightness_filter]
        };
        assert_eq!(rectangle.unfiltered_pixel_at(50, 50).unwrap(), rgba!(100, 100, 200, 255));
        assert_eq!(rectangle.filtered_pixel_at(50, 50).unwrap(), rgba!(200, 200, 255, 255));
    }
}

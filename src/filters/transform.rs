use crate::Filter;

#[derive(Default)]
pub struct TranslateFilter {
    pub x: isize,
    pub y: isize
}

impl<T> Filter<T> for TranslateFilter {
    fn filter_transform(&self, x: usize, y: usize) -> (usize, usize) {
        (x.wrapping_add_signed(-self.x), y.wrapping_add_signed(-self.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{layers::shapes::RectangleLayer, Layer, AlphaPixel, Rect};

    #[test]
    fn translate_test() {
        let translate_filter = Box::new(TranslateFilter {x: 10, y: -5});
        let rectangle = RectangleLayer {
            rect: Rect { x: 2, y: 8, width: 5, height: 6 },
            fill: AlphaPixel::<u8>::red(),
            filters: vec![translate_filter]
        };
        
        let bottom_right_pixel = rectangle.filtered_pixel_at(16, 8);

        assert!(bottom_right_pixel.is_some());
        assert_eq!(bottom_right_pixel.unwrap(), AlphaPixel::red());

        assert!(rectangle.filtered_pixel_at(3, 9).is_none());
        assert!(rectangle.filtered_pixel_at(13, 4).is_some());
    }
}

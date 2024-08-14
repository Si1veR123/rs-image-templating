use std::usize;

use num::traits::Inv;

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

pub struct MatrixTransform {
    pub matrix: [f32; 4],
    pub center_x: f32,
    pub center_y: f32
}

impl<T> Filter<T> for MatrixTransform {
    fn filter_transform(&self, x: usize, y: usize) -> (usize, usize) {
        let relative_x = x as f32 - self.center_x;
        let relative_y = y as f32 - self.center_y;

        let new_x = relative_x * self.matrix[0] + relative_y * self.matrix[1];
        let new_y = relative_x * self.matrix[2] + relative_y * self.matrix[3];

        let uncentered_new_x = new_x + self.center_x;
        let uncentered_new_y = new_y + self.center_y;

        // If coordinates are negative, then return usize::MAX (this can't be a valid coordinate)
        (
            (uncentered_new_x as i32).try_into().unwrap_or(usize::MAX),
            (uncentered_new_y as i32).try_into().unwrap_or(usize::MAX)
        )
    }
}

impl MatrixTransform {
    pub fn new(center_x: f32, center_y: f32) -> Self {
        Self { matrix: [1.0, 0.0, 0.0, 1.0], center_x, center_y }
    }
    
    /// This is the inverse matrix of the transformation to be applied to the layer.
    pub fn apply_matrix(mut self, matrix: &[f32; 4]) -> Self {
        let mut new_matrix = [0.0; 4];
        new_matrix[0] = self.matrix[0]*matrix[0] + self.matrix[1]*matrix[2];
        new_matrix[1] = self.matrix[0]*matrix[1] + self.matrix[1]*matrix[3];
        new_matrix[2] = self.matrix[2]*matrix[0] + self.matrix[3]*matrix[2];
        new_matrix[3] = self.matrix[2]*matrix[1] + self.matrix[3]*matrix[3];
        self.matrix = new_matrix;
        self
    }

    pub fn rotate(self, angle: f32) -> Self {
        let angle_rad = -angle.to_radians();
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();
        let matrix = [
            cos, -sin,
            sin, cos
        ];
        self.apply_matrix(&matrix)
    }

    pub fn scale(self, factor: f32) -> Self {
        self.apply_matrix(&[factor.inv(), 0.0, 0.0, factor.inv()])
    }

    pub fn scale_axis(self, scale_x: f32, scale_y: f32) -> Self {
        self.apply_matrix(&[scale_x.inv(), 0.0, 0.0, scale_y.inv()])
    }

    pub fn shear_x(self, factor: f32) -> Self {
        self.apply_matrix(&[1.0, -factor, 0.0, 1.0])
    }

    pub fn shear_y(self, factor: f32) -> Self {
        self.apply_matrix(&[1.0, 0.0, -factor, 1.0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{layers::shapes::RectangleLayer, AlphaPixel, Canvas, Layer, Rect, rgba};

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

    #[test]
    fn rotate_test() {
        let rotated_image = [
            rgba!(0, 0, 0, 0), rgba!(0, 0, 0, 0), rgba!(0, 0, 0, 0), rgba!(255, 0, 0, 255), rgba!(0, 0, 0, 0),
            rgba!(0, 0, 0, 0), rgba!(0, 0, 0, 0), rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255),
            rgba!(0, 0, 0, 0), rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255),
            rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255), rgba!(0, 0, 0, 0),
            rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255), rgba!(0, 0, 0, 0), rgba!(0, 0, 0, 0),
            rgba!(255, 0, 0, 255), rgba!(255, 0, 0, 255), rgba!(0, 0, 0, 0), rgba!(0, 0, 0, 0), rgba!(0, 0, 0, 0)
        ];
        let mut canvas = Canvas::<u8>::from_dimensions(5, 6);

        let rotate_filter = Box::new(MatrixTransform::new(5.0, 2.0).rotate(45.0));

        let rectangle = RectangleLayer { rect: Rect { x: 2, y: 2, width: 3, height: 6 }, fill: AlphaPixel::red(), filters: vec![rotate_filter] };
        canvas.add_layer(rectangle);
        let image = canvas.flatten();
        assert_eq!(image.get_pixels(), rotated_image);
    }
}

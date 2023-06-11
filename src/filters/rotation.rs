use super::LayerFilter;
use crate::{pixel::ImagePixels, parser::ParsedArgs};
use crate::colors::RGBAColor;

use std::collections::HashMap;

fn get_rotated_position(pixel_location: (i32, i32), width: i32, height: i32, new_width: i32, new_height: i32, angle: f64) -> (i32, i32) {
    let angle = angle.to_radians();

    // normalize to center
    let norm_x = (pixel_location.0 - (width/2)) as f64;
    let norm_y = ((height / 2) - pixel_location.1) as f64;

    let sin = angle.sin();
    let cos = angle.cos();

    // matrix rotation expanded
    let rot_x = norm_x * cos - norm_y * sin;
    let rot_y = norm_x * sin + norm_y * cos;

    let uncentered_rot_x = rot_x + (new_width as f64) / 2.0;
    let uncentered_rot_y = (new_height as f64) / 2.0 - rot_y;

    (uncentered_rot_x as i32, uncentered_rot_y as i32)
}

pub struct RotationFilter {
    args: HashMap<String, ParsedArgs>,
}

impl LayerFilter for RotationFilter {
    fn process(&self, pixels: &mut ImagePixels) {
        let degrees_anticlockwise = self.args.get("degrees")
            .expect("Degrees not provided for rotation")
            .as_float()
            .expect("Expected number for degrees.");

        // calculate new dimensions
        let new_w: u32;
        let new_h: u32;
        {
            let mut old_w = pixels.width() as f64;
            let mut old_h = pixels.height() as f64;
            
            let mut effective_degrees = degrees_anticlockwise % 180.0;
            if effective_degrees > 90.0 {
                (old_w, old_h) = (old_h, old_w);
                effective_degrees -= 90.0;
            }
            new_w = (old_h*effective_degrees.to_radians().sin() + old_w*effective_degrees.to_radians().cos()) as u32;
            new_h = (old_h*effective_degrees.to_radians().cos() + old_w*effective_degrees.to_radians().sin()) as u32;
        }

        // allocate memory for new rotated image
        let new_pixels_buf = vec![RGBAColor (0.0, 0.0, 0.0, 0.0); (new_h*new_w) as usize];
        let mut new_allocated_image = ImagePixels::from_pixels(new_w, new_pixels_buf);

        let new_w = new_w as i32;
        let new_h = new_h as i32;

        let old_w = pixels.width() as i32;
        let old_h = pixels.height() as i32;
        for (i, pixel) in new_allocated_image.get_pixels_mut().iter_mut().enumerate() {
            let i = i as i32;
            let x = i % new_w;
            let y = i / new_w;
            // do inverse of get rotated position (start from rotated position, rotate by anticlockwise angle, find original pixel)
            // results in a clockwise rotation
            let rotated_position = get_rotated_position((x, y), new_w, new_h, old_w, old_h, degrees_anticlockwise);
            if (rotated_position.0 < 0) || (rotated_position.1 < 0) {
                continue;
            }

            let pixel_to_copy = pixels.get_pixel_at(rotated_position.0 as u32, rotated_position.1 as u32);

            if pixel_to_copy.is_some() {
                *pixel = pixel_to_copy.unwrap().clone();
            }
        }

        *pixels = new_allocated_image;
    }

    fn new_with_args(args: HashMap<String, ParsedArgs>) -> Self {
        let args_string = HashMap::from_iter(
            args.into_iter()
        );
        Self { args: args_string }
    }
}

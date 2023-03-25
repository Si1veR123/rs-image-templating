use super::Layer;
use crate::{colors::*, pixel::ImagePixels, parser::ParsedArgs};

use std::collections::HashMap;

pub struct Rectangle {
    pixels: ImagePixels,
    position: (i32, i32)
}


impl Layer for Rectangle {
    fn new_layer(
        args: HashMap<String, ParsedArgs>
    ) -> Self {
        let width = args.get("width")
            .expect("Expected a width for Rectangle.")
            .as_int()
            .expect("Expected an integer for width.");

        assert!(width.is_positive());

        let height = args.get("height")
            .expect("Expected a height for Rectangle.")
            .as_int()
            .expect("Expected an integer for height.");

        assert!(height.is_positive());

        let background_color = args.get("background-color")
            .unwrap_or(&ParsedArgs::RGBAColor(WHITE))
            .as_rgb_color()
            .expect("Expected a color for background color.");
        
        let mut position = args.get("position")
            .unwrap_or(&ParsedArgs::Coord2D((0, 0)))
            .as_coord_2d()
            .expect("Expected a 2D coordinate for position.");

        let center = args.get("center")
            .unwrap_or(&ParsedArgs::Boolean(false))
            .as_bool()
            .expect("Expected bool for 'center'.");

        if center {
            position = (position.0 - width/2, position.1 - height/2);
        }

        let pixels = vec![vec![background_color; width as usize]; height as usize];
        let image_pixels: ImagePixels = pixels.into();

        Rectangle { pixels: image_pixels, position }
    }

    fn get_image(&mut self) -> &mut ImagePixels {
        &mut self.pixels
    }

    fn pixel_at(&self, x: u32, y: u32) -> Option<RGBAColor> {
        let x = x as i32;
        let y = y as i32;

        if (x < self.position.0) || (y < self.position.1) {
            return None
        }

        if (x >= self.position.0 + (self.pixels.width() as i32)) || (y >= self.position.1 + (self.pixels.height() as i32)) {
            return None
        }

        // in rectangle
        let rel_x = x - self.position.0;
        let rel_y = y - self.position.1;

        // impossible for rel_x or rel_y to be negative, as x > x position, and y > y position
        self.pixels.get_pixel_at(rel_x as usize, rel_y as usize).cloned()
    }
}

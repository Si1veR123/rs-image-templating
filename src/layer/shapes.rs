use super::Layer;
use crate::{colors::*, pixel::ImagePixels, parser::ParsedArgs};

use std::collections::HashMap;

pub struct Rectangle {
    pixels: ImagePixels
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
        
        let pixels = vec![vec![background_color; width as usize]; height as usize];

        let image_pixels: ImagePixels = pixels.into();

        Rectangle { pixels: image_pixels }
    }

    fn get_image(&mut self) -> &mut ImagePixels {
        &mut self.pixels
    }
}

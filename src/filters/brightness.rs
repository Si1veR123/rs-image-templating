use super::LayerFilter;
use crate::{colors::RGBAColor, pixel::ImagePixels, parser::ParsedArgs};

use std::collections::HashMap;

pub struct BrightnessFilter {
    args: HashMap<String, ParsedArgs>,
}

impl LayerFilter for BrightnessFilter {
    fn process(&self, pixels: &mut ImagePixels) {
        let brightness_multiplier = self.args.get("multiplier")
            .expect("Expected a brightness multiplier.")
            .as_float()
            .expect("Expected a float value for brightness multiplier.");
        
        for pixel in pixels.get_pixels_mut() {
            *pixel = RGBAColor(pixel.0 * brightness_multiplier, pixel.1 * brightness_multiplier, pixel.2 * brightness_multiplier, pixel.3)
        }
    }

    fn new_with_args(args: HashMap<String, ParsedArgs>) -> Self where Self: Sized {
        let args_string = HashMap::from_iter(
            args.into_iter()
        );
        Self { args: args_string }
    }
}

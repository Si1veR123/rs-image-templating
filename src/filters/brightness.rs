use super::LayerFilter;
use crate::{layer::Layer, colors::RGBAColor};

use std::collections::HashMap;

struct BrightnessFilter {

}

impl<L: Layer> LayerFilter<L> for BrightnessFilter {
    fn process(layer: &mut L, args: HashMap<&str, crate::parser::ParsedArgs>) {
        let brightness_multiplier = args.get("multiplier")
            .expect("Expected a brightness multiplier.")
            .as_float()
            .expect("Expected a float value for brightness multiplier.");
        
        let pixels = layer.get_image();
        for pixel in pixels.get_pixels_mut() {
            *pixel = RGBAColor(pixel.0 * brightness_multiplier, pixel.1 * brightness_multiplier, pixel.2 * brightness_multiplier)
        }
    }
}

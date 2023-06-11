use super::LayerFilter;
use crate::{colors::RGBAColor, pixel::ImagePixels, parser::ParsedArgs};

use std::collections::HashMap;

pub struct ChannelFilter {
    args: HashMap<String, ParsedArgs>,
}

impl LayerFilter for ChannelFilter {
    fn process(&self, pixels: &mut ImagePixels) {
        let channel_multiplier = self.args.get("filter").map(|x| x.as_rgb_color().expect("Expected a colour from with channels from 0-1 for filter."))
            .unwrap_or(RGBAColor(0.0, 0.0, 0.0, 0.0));

        for pixel in pixels.get_pixels_mut() {
            pixel.0 *= channel_multiplier.0;
            pixel.1 *= channel_multiplier.1;
            pixel.2 *= channel_multiplier.2;
            pixel.3 *= channel_multiplier.3;
        }
    }

    fn new_with_args(args: HashMap<String, ParsedArgs>) -> Self {
        let args_string = HashMap::from_iter(
            args.into_iter()
        );
        Self { args: args_string }
    }
}

use crate::{parser::{ParsedArgs, ConfigDeserializer}, pixel::ImagePixels};
use std::collections::HashMap;
use super::*;

pub trait LayerFilter {
    fn process(&self, pixels: &mut ImagePixels);
    fn new_with_args(args: HashMap<String, ParsedArgs>) -> Self where Self: Sized;
}

pub struct DefaultFilterDeserializer {}

impl ConfigDeserializer<Box<dyn LayerFilter>> for DefaultFilterDeserializer {
    fn from_str_and_args(from: &str, args: HashMap<String, ParsedArgs>) -> Box<dyn LayerFilter> {
        Box::new(match from {
            "brightness" => brightness::BrightnessFilter::new_with_args(args),
            _ => panic!("Invalid filter '{}' found.", from)
        })
    }
}

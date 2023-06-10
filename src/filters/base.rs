use crate::{parser::{ParsedArgs, ConfigDeserializer}, pixel::ImagePixels};
use std::collections::HashMap;
use super::*;

pub trait LayerFilter {
    fn process(&self, pixels: &mut ImagePixels);
    fn new_with_args(args: HashMap<String, ParsedArgs>) -> Self where Self: Sized;
}

macro_rules! filter_match {
    ($from:ident, $args:ident, $($name: literal => $path: ty),*) => {
        match $from {
            $($name => Box::new(<$path>::new_with_args($args)) as Box<dyn LayerFilter>,)*
            _ => panic!("Invalid filter name.")
        }
    };
}

pub struct DefaultFilterDeserializer;

impl ConfigDeserializer<Box<dyn LayerFilter>> for DefaultFilterDeserializer {
    fn from_str_and_args(from: &str, args: HashMap<String, ParsedArgs>) -> Box<dyn LayerFilter> {
        filter_match!(from, args, 
            "brightness" => brightness::BrightnessFilter, 
            "channel-filter" => channel_filter::ChannelFilter,
            "flip" => flip::FlipFilter,
            "rotate" => rotation::RotationFilter
        )
    }
}

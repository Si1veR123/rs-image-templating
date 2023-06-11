use std::collections::HashMap;
use crate::{parser::{ParsedArgs, ConfigDeserializer}, pixel::ImagePixels, filters::LayerFilter, colors::RGBAColor};
use super::shapes;
use super::images;
#[cfg(feature = "text")]
use super::text;

pub trait Layer {
    fn get_image(&mut self) -> &mut ImagePixels;
    fn new_layer(
        args: HashMap<String, ParsedArgs>
    ) -> Self
        where Self: Sized;
    fn apply_filters(&mut self, filters: &Vec<Box<dyn LayerFilter>>) {
        for filter in filters {
            filter.process(self.get_image())
        }
    }
    fn pixel_at(&self, x: u32, y: u32) -> Option<RGBAColor>;
}

macro_rules! layer_match {
    ($from:ident, $args:ident, $($name: literal => $path: ty),*) => {
        match $from {
            $($name => Box::new(<$path>::new_layer($args)) as Box<dyn Layer>,)*
            _ => panic!("Invalid layer name.")
        }
    };
}

pub struct DefaultLayerDeserializer;

impl ConfigDeserializer<Box<dyn Layer>> for DefaultLayerDeserializer {
    #[cfg(feature = "text")]
    fn from_str_and_args(from: &str, args: HashMap<String, ParsedArgs>) -> Box<dyn Layer> {
        layer_match!(from, args,
            "rectangle" => shapes::Rectangle,
            "image" => images::Image,
            "text" => text::Text
        )
    }
    #[cfg(not(feature = "text"))]
    fn from_str_and_args(from: &str, args: HashMap<String, ParsedArgs>) -> Box<dyn Layer> {
        layer_match!(from, args,
            "rectangle" => shapes::Rectangle,
            "image" => images::Image
        )
    }
}

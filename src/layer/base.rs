use std::collections::HashMap;
use crate::{parser::{ParsedArgs, ConfigDeserializer}, pixel::ImagePixels, filters::LayerFilter, colors::RGBAColor};
use super::shapes;

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

pub struct DefaultLayerDeserializer {}

impl ConfigDeserializer<Box<dyn Layer>> for DefaultLayerDeserializer {
    fn from_str_and_args(from: &str, args: HashMap<String, ParsedArgs>) -> Box<dyn Layer> {
        Box::new(match from {
            "rectangle" => shapes::Rectangle::new_layer(args),
            _ => panic!("Invalid layer name")
        })
    }
}

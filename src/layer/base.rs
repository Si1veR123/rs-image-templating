use std::collections::HashMap;

use crate::{parser::ParsedArgs, pixel::ImagePixels};

pub trait Layer {
    fn init(args: HashMap<&str, ParsedArgs>) -> Self;
    fn get_image(&mut self) -> &mut ImagePixels;
}

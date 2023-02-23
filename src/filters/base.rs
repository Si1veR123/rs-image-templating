use crate::{layer::Layer, parser::ParsedArgs};
use std::collections::HashMap;

pub trait LayerFilter<T: Layer> {
    fn process(layer: &mut T, args: HashMap<&str, ParsedArgs>);
}
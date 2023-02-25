use crate::{layer::Layer, filters::LayerFilter};
use crate::parser::ParsedArgs;
use super::{LayerParser, ConfigDeserializer, LayerFilterTuple};
use std::io::BufRead;
use std::collections::HashMap;
use toml::{from_str, Value};

fn toml_to_args_hashmap(map: toml::map::Map<String, Value>) -> HashMap<String, ParsedArgs> {
    let mut parsed_args_hashmap = HashMap::with_capacity(map.len());
    for (k, v) in map {
        let new_val = <Value as TryInto<ParsedArgs>>::try_into(v);
        if new_val.is_ok() {
            parsed_args_hashmap.insert(k, new_val.unwrap());
        }
    }
    parsed_args_hashmap
}

pub struct TomlLayerParser {}

impl LayerParser for TomlLayerParser {
    fn parse<R: BufRead, L: ConfigDeserializer<Box<dyn Layer>>, F: ConfigDeserializer<Box<dyn LayerFilter>>>(mut reader: R) -> LayerFilterTuple {
        let mut buf = String::new();
        let _len = reader.read_to_string(&mut buf).expect("Invalid chars in file.");
        let toml_parsed: Value = from_str(&buf).expect("Invalid toml.");

        let layers = toml_parsed.as_table()
            .expect("Root of toml isn't table.")
            .get("layer")
            .expect("No layers found.")
            .as_array()
            .expect("Layer isn't an array of tables. Ensure layer has double square brackets.")
            .clone();

        let mut layer_buf = Vec::with_capacity(layers.len());
        
        for layer in layers {
            let mut table = layer.as_table()
                .expect("Layer isn't an array of tables.")
                .clone();

            let filters = table.remove("filter");

            let filters_vec = match filters {
                Some(filters) => filters.as_array().expect("filter is formatted incorrectly.").clone(),
                None => vec![]
            };

            let mut filter_objects = Vec::with_capacity(filters_vec.len());
            for filter in filters_vec {
                let mut filter = filter.as_table().expect("filter is formatted incorrectly.").clone();

                let filter_type_unchecked = filter.remove("type");
                let filter_type_value = filter_type_unchecked.expect("No type found on filter.");
                let filter_type = filter_type_value.as_str().expect("Invalid 'type' type on filter").trim_matches('"');

                let parsed_args = toml_to_args_hashmap(filter);

                let filter_deserialized = F::from_str_and_args(filter_type, parsed_args);
                filter_objects.push(filter_deserialized);
            }

            let mut parsed_args_hashmap = toml_to_args_hashmap(table);

            let type_arg = parsed_args_hashmap.remove("type").expect("'type' not found on layer.");
            let layer_type = type_arg.as_str().expect("Invalid 'type' type on layer :)");
            let trimmed = layer_type.trim_matches('"');

            let layer = L::from_str_and_args(&trimmed, parsed_args_hashmap);
            layer_buf.push((layer, filter_objects));
        }

        layer_buf
    }
}

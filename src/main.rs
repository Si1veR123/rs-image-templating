use image_template:: {
    parser::{toml::TomlLayerParser, LayerParser},
    layer::DefaultLayerDeserializer,
    filters::DefaultFilterDeserializer
};

use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("example_layers_config.toml").unwrap();
    let buf_reader = BufReader::new(file);

    let mut parsed = TomlLayerParser::parse::<_, DefaultLayerDeserializer, DefaultFilterDeserializer>(buf_reader);

    for (layer, filters) in &mut parsed {
        layer.apply_filters(filters)
    }
}

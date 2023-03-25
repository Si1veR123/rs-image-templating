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

    let canvas = TomlLayerParser::parse::<_, DefaultLayerDeserializer, DefaultFilterDeserializer>(buf_reader);
    let final_image = canvas.aggregate_layers_into_image_lib();
    let r = final_image.save("output.png");
    println!("{:?}", r);
}

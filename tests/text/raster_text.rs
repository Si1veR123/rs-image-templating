use image_template::{Canvas, layers::text::{layout::TextLayout, TextLayer, TextSettings}, AlphaPixel, Image, ImageFormat};
use crate::text::get_font;

#[test]
fn rasterize_basic() {
    let reference_image: Image<u8> = Image::load_from_memory(include_bytes!("raster_text.png"), ImageFormat::Png).unwrap();

    let mut canvas = Canvas::from_dimensions(600, 85);

    let text_layer = TextLayer::try_new(
        TextSettings {
            size: 30.0,
            fill: AlphaPixel::red(),
            layout: TextLayout::default(),
            text: String::from("The quick brown fox jumps over a lazy dog.\nSphinx of black quartz, judge my vow."),
            font: get_font()
        }, 
        20,
        5
    ).unwrap();

    canvas.add_layer(text_layer);
    let image = canvas.flatten();

    assert!(image.get_pixels() == reference_image.get_pixels(), "Text rasterized images are different.");
}
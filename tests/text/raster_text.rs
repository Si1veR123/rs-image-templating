use image::GenericImageView;
use image_template::{Canvas, layers::text::{layout::TextLayout, TextLayer, TextSettings}, AlphaPixel, rgba, Image};

use crate::text::get_font;

#[test]
fn rasterize_basic() {
    // TODO: create an `Image::from_png`
    let reference = image::load_from_memory_with_format(include_bytes!("rasterize_basic.png"), image::ImageFormat::Png).unwrap();
    let reference_image = Image::from_pixels(
        reference.pixels().map(|p| rgba!(p.2.0[0], p.2.0[1], p.2.0[2], p.2.0[3])).collect(),
        600
    ).unwrap();

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

    assert_eq!(image.get_pixels(), reference_image.get_pixels());
}
use image_template::filters::transform::TranslateFilter;
use image_template::layers::text::layout::{LayoutDirection, SpacingMode, TextLayout};
use image_template::layers::text::{TextLayer, TextSettings};
use image_template::pixels::pixel::AlphaPixel;
use image_template::rect::Rect;
use image_template::{
    canvas::Canvas,
    layers::shapes::RectangleLayer
};

fn main() {
    let mut canvas: Canvas<u8> = Canvas::from_dimensions(500, 500);
    let font = include_bytes!(r"C:\Windows\Fonts\Impact.ttf") as &[u8];

    let text = TextLayer::new(
        TextSettings {
            size: 50.0,
            fill: AlphaPixel { r: 0, g: 255, b: 0, a: 155 },
            layout: TextLayout { direction: LayoutDirection::TopToBottom, line_spacing: SpacingMode::Scale(1.0), glyph_spacing: SpacingMode::Scale(1.0), use_kern: false  },
            text: String::from("Testingsentence\ncol"),
            font: fontdue::Font::from_bytes(font, fontdue::FontSettings { collection_index: 0, scale: 100.0, load_substitutions: true  }).unwrap()
        },
        0,
        0
    ).unwrap();

    canvas.add_layer(text);
    let result = canvas.flatten();
    let _ = result.save("test.png", image::ImageFormat::Png);
}

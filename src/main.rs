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
    let mut canvas: Canvas<u8> = Canvas::from_dimensions(1000, 1000);
    let font = include_bytes!(r"C:\Windows\Fonts\Arial.ttf") as &[u8];

    let text = TextLayer::new(
        TextSettings {
            size: 50.0,
            fill: AlphaPixel { r: 0, g: 255, b: 0, a: 155 },
            layout: TextLayout { direction: LayoutDirection::TopToBottom, line_spacing: SpacingMode::Constant(300.0), glyph_spacing: SpacingMode::Scale(2.0), use_kern: true  },
            text: String::from("Testingsentence\ncol"),
            font: fontdue::Font::from_bytes(font, fontdue::FontSettings { collection_index: 0, scale: 100.0, load_substitutions: true  }).unwrap()
        },
        50,
        50
    );

    //let rect = RectangleLayer::new(AlphaPixel { r: 255, g: 255, b: 255, a: 255 }, Rect { x: 500, y: 500, width: 500, height: 500 });

    canvas.add_layer(text);
    //canvas.add_layer(rect);
    let result = canvas.flatten();
    let _ = result.save("test.png", image::ImageFormat::Png);
}

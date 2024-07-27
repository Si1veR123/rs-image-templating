use image_template::filters::transform::TranslateFilter;
use image_template::pixels::pixel::AlphaPixel;
use image_template::rect::Rect;
use image_template::{
    canvas::Canvas,
    layers::shapes::RectangleLayer
};

fn main() {
    let mut canvas: Canvas<u8> = Canvas::from_dimensions(10, 10);
    let mut rectangle = RectangleLayer::new(AlphaPixel { r: 255, g: 255, b: 255, a: 255 }, Rect { x: 3, y: 3, width: 5, height: 5 });
    let translate = TranslateFilter {x: 3, y: 3};
    rectangle.filters.push(Box::new(translate));

    canvas.add_layer(rectangle);
    let result = canvas.flatten();
    println!("{:?}", result);
}

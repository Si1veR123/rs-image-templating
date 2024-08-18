use image_template::{filters::transform::MatrixTransform, layers::image::ImageLayer, AlphaPixel, Canvas, Image, ImageFormat};


#[test]
fn all_matrix_transform() {
    let reference_image = Image::load_from_memory(include_bytes!("transform.png"), ImageFormat::Png).unwrap();

    let matrix_filter = Box::new(MatrixTransform::new(50.0, 37.0)
        .shear_x(-0.5)
        .rotate(90.0)
        .scale_axis(2.0, 1.5)
        .shear_y(-1.0)
        .scale(1.7)
        .rotate(20.0));

    let mut canvas: Canvas<u8> = Canvas::from_dimensions(100, 75);
    let image = Image::from_function(25, 15, |x, y| AlphaPixel { r: x as u8 * 4, g: y as u8 * 4, b: x as u8 * 4, a: 255 });
    let image_layer = ImageLayer { im: image, filters: vec![matrix_filter], x: 38, y: 30 };
    canvas.add_layer(image_layer);
    let result = canvas.flatten();

    assert!(reference_image.get_pixels() == result.get_pixels(), "Matrix transform images are different.");
}

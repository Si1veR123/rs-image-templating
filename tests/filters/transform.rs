use image::GenericImageView;
use image_template::{filters::transform::MatrixTransform, layers::image::ImageLayer, AlphaPixel, Canvas, Image, rgba};


#[test]
fn all_matrix_transform() {
    let reference = image::load_from_memory_with_format(include_bytes!("transform.png"), image::ImageFormat::Png).unwrap();
    let reference_image = Image::from_pixels(
        reference.pixels().map(|p| rgba!(p.2.0[0], p.2.0[1], p.2.0[2], p.2.0[3])).collect(),
        1000
    ).unwrap();

    let matrix_filter = Box::new(MatrixTransform::new(500.0, 375.0)
        .shear_x(-0.5)
        .rotate(90.0)
        .scale_axis(2.0, 1.5)
        .shear_y(-1.0)
        .scale(1.7)
        .rotate(20.0));

    let mut canvas: Canvas<u8> = Canvas::from_dimensions(1000, 750);
    let image = Image::from_function(250, 150, |x, y| AlphaPixel { r: x as u8, g: y as u8, b: x as u8, a: 255 });
    let image_layer = ImageLayer { im: image, filters: vec![matrix_filter], x: 380, y: 300 };
    canvas.add_layer(image_layer);
    let result = canvas.flatten();
    
    assert!(reference_image.get_pixels() == result.get_pixels(), "Matrix transform images are different.");
}

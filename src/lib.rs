//! This crate provides a way to generate images from layers. Examples of layers are shapes, images and text.
//! It is based around a [`Canvas`], that layers are added to. Once all layers have been added and the necessary
//! processing is complete, a Canvas can be converted into a bitmap [`Image`].
//! 
//! # Layers
//! Layers are any objects that implement the [`Layer`] trait. This allows the developer to create their own layers
//! for graphics not included in this library. Some examples of layers are [`ImageLayer`](crate::layers::image::ImageLayer)
//! and [`TextLayer`](crate::layers::text::TextLayer).
//! 
//! # Filters
//! Filters can be added to layers to manipulate their output. They implement the [`Filter`] trait.
//! A filter can modify the pixel, or the coordinate that the pixel is sampled from.
//! Some examples of filters provided by this library are [`MatrixTransform`](crate::filters::transform::MatrixTransform)
//! and [`BrightnessFilter`](crate::filters::brightness::BrightnessFilter).
//! 
//! # Image
//! [`Image`] is a bitmap image which stores a `Vec` of pixels. This is the main way that images are represented in this library.
//! Image implements `AsRef<[u8]>`, which can be used to get a slice of bytes representing the pixels.
//! Each pixel is of the type [`AlphaPixel<T>`]. This is a pixel with RGBA channels. `T` must implement [`PixelChannel`] for most usages.
//! 
//! # Basic Example
//! 
//! ```rust,no_run
//! use image_template::{Canvas, Image, AlphaPixel, ImageFormat, layers::image::ImageLayer};
//! 
//! let mut canvas: Canvas<u8> = Canvas::from_dimensions(1000, 1000);
//!
//! let image = Image::from_function(500, 500, |x, y| AlphaPixel { r: x as u8, g: y as u8, b: 255, a: 255 });
//! let image_layer = ImageLayer::new(image, 0, 0);
//!
//! canvas.add_layer(image_layer);
//!
//! let final_image = canvas.flatten();
//! final_image.save("test.png", ImageFormat::Png).expect("Error saving image.");
//! ```

#[cfg(feature = "image-crate")]
pub use image::ImageFormat;

mod canvas;
pub use canvas::Canvas;

mod rect;
pub use rect::Rect;

pub mod bitmap;
pub use bitmap::{
    pixel::{
        AlphaPixel, PixelChannel
    },
    image::Image,
    blending::BlendingMethod
};

pub mod layers;
pub use layers::Layer;

pub mod filters;
pub use filters::Filter;
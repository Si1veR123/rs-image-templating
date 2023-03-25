use crate::colors::{RGBAColor, over_operator};
use crate::layer::Layer;
use crate::parser::LayerFilterTuple;

use image::{RgbaImage, Rgba};

pub struct Canvas {
    layers: Vec<Box<dyn Layer>>,
    width: u32,
    height: u32
}

impl Canvas {
    pub fn from_layers_and_filters(mut layers: LayerFilterTuple, width: u32, height: u32) -> Self {
        for (layer, filters) in &mut layers {
            layer.apply_filters(filters)
        }

        let new_layers_vec: Vec<_> = layers.into_iter().map(|e| e.0).collect();

        Self { layers: new_layers_vec, width, height }
    }

    fn aggregate_pixel(&self, x: u32, y: u32) -> RGBAColor {
        // combine pixels from all layers
        let mut running_pixel = RGBAColor(255.0, 255.0, 255.0, 0.0);
        for layer in &self.layers {
            let layer_pixel = layer.pixel_at(x, y);
            if layer_pixel.is_some() {
                running_pixel = over_operator(&layer_pixel.unwrap(), &running_pixel)
            }
        }

        running_pixel
    }

    pub fn aggregate_layers_into_image_lib(&self) -> RgbaImage {
        let img = RgbaImage::from_fn(self.width, self.height, |x, y| {
            let p = self.aggregate_pixel(x, y);
            Rgba([p.0 as u8, p.1 as u8, p.2 as u8, p.3 as u8])
        });
        img
    }
}

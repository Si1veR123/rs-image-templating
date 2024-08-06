use crate::{
    layers::Layer,
    pixels::{
        blending::BlendingMethod, image::Image, pixel::{AlphaPixel, PixelChannel}
    }
};

pub struct Canvas<T> {
    pub layers: Vec<Box<dyn Layer<T>>>,
    pub width: usize,
    pub height: usize
}

impl<T: PixelChannel> Canvas<T> {
    pub fn from_dimensions(width: usize, height: usize) -> Self {
        Self { layers: vec![], width, height }
    }

    pub fn add_layer<L: Layer<T> + 'static>(&mut self, layer: L) {
        self.layers.push(Box::new(layer));
    }

    pub fn pixel_at(&mut self, x: usize, y: usize) -> AlphaPixel<T> {
        let mut running_pixel = AlphaPixel {r: T::max_value(), g: T::max_value(), b: T::max_value(), a: T::zero()};
        for layer in &self.layers {
            let layer_pixel = layer.filtered_pixel_at(x, y);

            if let Some(p) = layer_pixel {
                running_pixel = BlendingMethod::OverOperator.blend(p, running_pixel);
            }
        }

        running_pixel
    }

    pub fn flatten(&mut self) -> Image<T> {
        let mut pixels = Vec::with_capacity(self.width*self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                pixels.push(self.pixel_at(col, row));
            }
        }
        // `pixels.len() = self.width*self.height`
        Image::from_pixels(pixels, self.width).unwrap()
    }
}

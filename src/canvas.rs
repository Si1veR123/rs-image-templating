use crate::{
    layers::Layer,
    pixels::{
        blending::BlendingMethod, image::Image, pixel::{AlphaPixel, PixelChannel}
    }
};

pub struct Canvas<T> {
    pub layers: Vec<Box<dyn Layer<T>>>,
    pub background: AlphaPixel<T>,
    pub width: usize,
    pub height: usize
}

impl<T: PixelChannel> Canvas<T> {
    pub fn from_dimensions(width: usize, height: usize) -> Self {
        Self { layers: vec![], background: AlphaPixel::default(), width, height }
    }

    pub fn add_layer<L: Layer<T> + 'static>(&mut self, layer: L) {
        self.layers.push(Box::new(layer));
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> AlphaPixel<T> {
        let mut running_pixel = self.background;
        for layer in &self.layers {
            let layer_pixel = layer.filtered_pixel_at(x, y);

            if let Some(p) = layer_pixel {
                running_pixel = BlendingMethod::OverOperator.blend(running_pixel, p);
            }
        }

        running_pixel
    }

    pub fn flatten(&self) -> Image<T> {
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

use crate::{
    Layer,
    Image,
    AlphaPixel,
    PixelChannel,
    BlendingMethod
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

    pub fn combined_pixel_at(&self, x: usize, y: usize) -> AlphaPixel<T> {
        let mut running_pixel = self.background;
        for layer in &self.layers {
            let layer_pixel = layer.filtered_pixel_at(x, y);

            if let Some(p) = layer_pixel {
                running_pixel = BlendingMethod::Over.blend(running_pixel, p);
            }
        }

        running_pixel
    }

    pub fn flatten(&self) -> Image<T> {
        let mut pixels = Vec::with_capacity(self.width*self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                pixels.push(self.combined_pixel_at(col, row));
            }
        }
        // `pixels.len() = self.width*self.height`
        Image::from_pixels(pixels, self.width).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{layers::shapes::RectangleLayer, rect::Rect, rgba};

    #[test]
    fn background() {
        let background = rgba!(100u8, 100, 100, 255);
        let mut canvas = Canvas::from_dimensions(10, 10);
        canvas.background = background;

        for row in 0..10 {
            for col in 0..10 {
                assert_eq!(canvas.combined_pixel_at(col, row), background)
            }
        }
    }

    fn half_colored_canvas() -> Canvas<u8> {
        let mut canvas: Canvas<u8> = Canvas::from_dimensions(10, 10);
        let top_half_rect = RectangleLayer::new(AlphaPixel::red(), Rect { x: 0, y: 0, width: 10, height: 5 });
        let bottom_half_rect = RectangleLayer::new(AlphaPixel::blue(), Rect { x: 0, y: 5, width: 10, height: 5 });
        canvas.add_layer(top_half_rect);
        canvas.add_layer(bottom_half_rect);
        canvas
    }

    #[test]
    fn pixel_at() {
        let canvas = half_colored_canvas();

        for row in 0..10 {
            for col in 0..10 {
                assert_eq!(canvas.combined_pixel_at(col, row), if row < 5 { AlphaPixel::red() } else { AlphaPixel::blue() })
            }
        }
    }

    #[test]
    fn flatten() {
        let canvas = half_colored_canvas();
        let image = canvas.flatten();

        for row in 0..10 {
            for col in 0..10 {
                assert_eq!(image.pixel_at(col, row).unwrap(), if row < 5 { AlphaPixel::red() } else { AlphaPixel::blue() })
            }
        }
    }
}

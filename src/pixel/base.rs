use crate::colors::RGBAColor;

pub struct ImagePixels {
    width: usize,
    pixels: Vec<RGBAColor>
}

impl ImagePixels {
    pub fn from_pixel_rows(rows: Vec<Vec<RGBAColor>>) -> Self {
        let width = rows.get(0).expect("Can't construct ImagePixels with 0 height").len();

        let pixels: Vec<RGBAColor> = rows.into_iter().flatten().collect();

        Self { width, pixels }
    }

    pub fn from_pixels(width: usize, pixels: Vec<RGBAColor>) -> Self {
        Self { width, pixels }
    }

    pub fn as_pixel_rows(&self) -> Vec<Vec<RGBAColor>> {
        let mut new_pixels_buffer = Vec::with_capacity(self.height());
        let rows = self.pixels.windows(self.width);

        for row in rows {
            new_pixels_buffer.push(row.to_vec())
        }

        new_pixels_buffer
    }

    pub fn get_pixels(&self) -> &Vec<RGBAColor> {
        &self.pixels
    }

    pub fn get_pixels_mut(&mut self) -> &mut Vec<RGBAColor> {
        &mut self.pixels
    }

    pub fn height(&self) -> usize {
        self.pixels.len() / (self.width as usize)
    }
} 

impl From<Vec<Vec<RGBAColor>>> for ImagePixels {
    fn from(pixels: Vec<Vec<RGBAColor>>) -> Self {
        ImagePixels::from_pixel_rows(pixels)
    }
}

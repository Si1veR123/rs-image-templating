use crate::colors::RGBAColor;

pub struct ImagePixels {
    width: u32,
    pixels: Vec<RGBAColor>
}

impl ImagePixels {
    pub fn from_pixel_rows(rows: Vec<Vec<RGBAColor>>) -> Self {
        let width = rows.get(0).expect("Can't construct ImagePixels with 0 height").len() as u32;

        let pixels: Vec<RGBAColor> = rows.into_iter().flatten().collect();

        Self { width, pixels }
    }

    pub fn from_pixels(width: u32, pixels: Vec<RGBAColor>) -> Self {
        Self { width, pixels }
    }

    pub fn as_raw(self) -> Vec<RGBAColor> {
        self.pixels
    }

    pub fn as_pixel_rows(&self) -> Vec<Vec<RGBAColor>> {
        let mut new_pixels_buffer = Vec::with_capacity(self.height() as usize);
        let rows = self.pixels.chunks(self.width as usize);

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

    pub fn get_pixel_at(&self, x: u32, y: u32) -> Option<&RGBAColor> {
        if (x >= self.width) || (y >= self.height()) {
            return None
        }
        self.pixels.get((x + y * self.width) as usize)
    }

    pub fn get_pixel_at_mut(&mut self, x: u32, y: u32) -> Option<&mut RGBAColor> {
        if (x >= self.width) || (y >= self.height()) {
            return None
        }
        self.pixels.get_mut((x + y * self.width) as usize)
    }

    pub fn height(&self) -> u32 {
        self.pixels.len() as u32 / (self.width)
    }

    pub fn width(&self) -> u32 {
        self.width
    }
} 

impl From<Vec<Vec<RGBAColor>>> for ImagePixels {
    fn from(pixels: Vec<Vec<RGBAColor>>) -> Self {
        ImagePixels::from_pixel_rows(pixels)
    }
}

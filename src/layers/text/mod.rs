pub mod layout;

use crate::{filters::Filter, layers::{text::layout::{TextLayout, LayoutIter}, Layer}, pixels::{blending::BlendingMethod, image::Image, pixel::{AlphaPixel, PixelChannel}}, rect::Rect};
use fontdue::Font;
use layout::LayoutError;
use std::{collections::HashMap, iter::repeat};

#[derive(Clone)]
pub struct TextSettings<T: PixelChannel> {
    pub size: f32,
    pub fill: AlphaPixel<T>,

    pub layout: TextLayout,

    pub text: String,
    pub font: Font,
}

type SignedCoord = (isize, isize);
type GlyphPositionMapping = HashMap<char, Vec<SignedCoord>>;

impl<T: PixelChannel> TextSettings<T> {
    /// Return a HashMap mapping a glyph to a Vec of coordinates, the minimum coordinate, and the maximum coordinate
    /// 
    /// Coordinates are `isize` as some glyphs may have negative coordinates.
    /// The minimum coordinates can be used to shift all coordinates to be positive.
    fn glyph_positions(&self) -> Result<(GlyphPositionMapping, SignedCoord, SignedCoord), LayoutError> {
        let mut positions: HashMap<char, Vec<(isize, isize)>> = HashMap::with_capacity(self.text.len());
        let mut minimum_coord = (0, 0);
        let mut maximum_coord = (0, 0);

        for layout in LayoutIter::new(self) {
            let (glyph, glyph_x, glyph_y) = layout?;

            positions.entry(glyph)
                .and_modify(|coordinates| coordinates.push((glyph_x, glyph_y)))
                .or_insert_with(|| vec![(glyph_x, glyph_y)]);

            let glyph_metrics = self.font.metrics(glyph, self.size);

            let glyph_greatest_coord = (glyph_x + glyph_metrics.width as isize, glyph_y + glyph_metrics.height as isize);
            maximum_coord.0 = maximum_coord.0.max(glyph_greatest_coord.0);
            maximum_coord.1 = maximum_coord.1.max(glyph_greatest_coord.1);

            minimum_coord.0 = minimum_coord.0.min(glyph_x);
            minimum_coord.1 = minimum_coord.1.min(glyph_y);
        }

        Ok((positions, minimum_coord, maximum_coord))
    }

    /// Create a rasterized image from the text settings
    pub fn raster_from_settings(&self) -> Result<Image<T>, LayoutError> {
        let (glyph_positions, minimum_coord, maximum_coord) = self.glyph_positions()?;
        let final_size = ((maximum_coord.0 - minimum_coord.0) as usize, (maximum_coord.1 - minimum_coord.1) as usize);

        let mut final_image = Image::from_pixels(repeat(AlphaPixel::default()).take(final_size.0*final_size.1).collect(), final_size.0).unwrap();

        for (glyph, coordinates) in glyph_positions.iter() {
            let (metrics, raster_pixels) = self.font.rasterize(*glyph, self.size);
            let raster_pixels_rgba = raster_pixels
                .iter()
                .map(|p| AlphaPixel { a: T::from_u8(*p).unwrap(), ..self.fill })
                .collect();
            let raster_image = Image::from_pixels(raster_pixels_rgba, metrics.width).unwrap();
            
            for coordinate in coordinates {
                final_image.draw_subimage(
                    &raster_image,
                    (coordinate.0 - minimum_coord.0) as usize, 
                    (coordinate.1 - minimum_coord.1) as usize,
                    BlendingMethod::OverOperator
                ).unwrap();
            }
        }

        Ok(final_image)
    }
}

/// A layer representing text. This may be a single character, a single line, or multiple lines.
pub struct TextLayer<T: PixelChannel> {
    settings: TextSettings<T>,
    rasterized: Image<T>,
    pub x: usize,
    pub y: usize,
    pub filters: Vec<Box<dyn Filter<T>>>
}

impl<T: PixelChannel> TextLayer<T> {
    pub fn try_new(settings: TextSettings<T>, x: usize, y: usize) -> Result<Self, LayoutError> {
        let raster = settings.raster_from_settings()?;
        Ok(Self { settings, rasterized: raster, x, y, filters: vec![] })
    }

    pub fn get_settings(&self) -> &TextSettings<T> {
        &self.settings
    }

    pub fn set_settings(&mut self, settings: TextSettings<T>) -> Result<(), LayoutError> {
        self.settings = settings;
        self.rasterized = self.settings.raster_from_settings()?;
        Ok(())
    }
}

impl<T: PixelChannel> Layer<T> for TextLayer<T> {
    fn get_rect(&self) -> Rect {
        Rect { x: self.x, y: self.y, width: self.rasterized.get_width(), height: self.rasterized.get_height() }
    }

    fn get_filters(&self) -> &[Box<dyn Filter<T>>] {
        &self.filters
    }

    fn unfiltered_pixel_at_unchecked(&self, x: usize, y: usize) -> AlphaPixel<T> {
        self.rasterized.pixel_at(x-self.x, y-self.y).unwrap()
    }
}

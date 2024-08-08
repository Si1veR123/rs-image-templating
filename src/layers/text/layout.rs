use std::str::Chars;
use fontdue::Metrics;
use thiserror::Error;
use crate::pixels::pixel::PixelChannel;
use super::TextSettings;

pub const DEFAULT_VERTICAL_SPACING: f32 = 10.0;

#[derive(Debug, Error)]
pub enum LayoutError {
    #[error("Font doesn't have line spacing. Use constant line spacing, or another font.")]
    MissingLineSpacing
}

#[derive(PartialEq, Clone, Copy)]
pub enum LayoutDirection {
    LeftToRight,
    TopToBottom
}

#[derive(Clone, Copy, PartialEq)]
pub enum SpacingMode {
    Scale(f32),
    Constant(f32)
}

#[derive(Clone, Copy, PartialEq)]
pub struct TextLayout {
    pub direction: LayoutDirection,
    pub line_spacing: SpacingMode,
    pub glyph_spacing: SpacingMode,
    pub use_kern: bool
}

impl Default for TextLayout {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::LeftToRight,
            line_spacing: SpacingMode::Scale(1.0),
            glyph_spacing: SpacingMode::Scale(1.0),
            use_kern: true
        }
    }
}
pub struct LayoutIter<'a, T: PixelChannel> {
    text: Chars<'a>,

    // Previous char, x/y (depending on direction) coordinate of the next origin position
    prev_data: Option<(char, isize)>,

    settings: &'a TextSettings<T>,
    row: usize
}

impl<'a, T: PixelChannel> LayoutIter<'a, T> {
    pub fn new(settings: &'a TextSettings<T>) -> Self {
        Self { text: settings.text.chars(), prev_data: None, settings, row: 0 }
    }

    /// Only used for left to right layouts. Calculate the origin for `next_char` using scaled kerning values.
    fn calculate_kerned_origin(&self, origin: isize, prev_char: char, next_char: char) -> isize {
        let kern = self.settings.font.horizontal_kern(prev_char, next_char, self.settings.size).unwrap_or(0.0);

        if let SpacingMode::Scale(scale) = self.settings.layout.glyph_spacing {
            origin + (kern * scale) as isize
        } else {
            origin + kern as isize
        }
    }

    /// Calculate the baseline of the next character
    fn calculate_baseline(&self, metrics: &Metrics) -> Result<isize, LayoutError> {
        match self.settings.layout.direction {
            LayoutDirection::LeftToRight => match self.settings.layout.line_spacing {
                SpacingMode::Constant(spacing) => Ok((spacing * (self.row + 1) as f32) as isize),
                SpacingMode::Scale(scale) => match self.settings.font.horizontal_line_metrics(self.settings.size) {
                    Some(line_metrics) => Ok((line_metrics.new_line_size * (self.row + 1) as f32 * scale) as isize),
                    None => Err(LayoutError::MissingLineSpacing)
                }
            },
            LayoutDirection::TopToBottom => match self.prev_data {
                Some((_prev_char, next_origin_y)) => Ok(next_origin_y),
                // Baseline of first character in a column
                None => Ok(metrics.height as isize - metrics.ymin as isize)
            },
        }
    }

    fn calculate_origin_x(&self, next_char: char) -> Result<isize, LayoutError> {
        match self.settings.layout.direction {
            LayoutDirection::LeftToRight => match self.prev_data {
                Some((prev_char, next_origin_x)) => match self.settings.layout.use_kern {
                    true => Ok(self.calculate_kerned_origin(next_origin_x, prev_char, next_char)),
                    false => Ok(next_origin_x)
                },
                None => Ok(0),
            }
            LayoutDirection::TopToBottom => match self.settings.layout.line_spacing {
                SpacingMode::Scale(scale) => match self.settings.font.vertical_line_metrics(self.settings.size) {
                    Some(line_metrics) => Ok((line_metrics.new_line_size * self.row as f32 * scale) as isize),
                    None => Err(LayoutError::MissingLineSpacing)
                },
                SpacingMode::Constant(spacing) => Ok((self.row as f32 * spacing) as isize),
            },
        }
    }
}

impl<'a, T: PixelChannel> Iterator for LayoutIter<'a, T> {
    type Item = Result<(char, isize, isize), LayoutError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_char = self.text.next()?;
        
        while next_char == '\n' {
            self.row += 1;
            self.prev_data = None;
            next_char = self.text.next()?;
        }

        let metrics = self.settings.font.metrics(next_char, self.settings.size);

        // Glyph x is the coordinate that the rasterized glyph should be drawn at.
        // It is an offset from the origin by `metrics.xmin`.
        let glyph_x = match self.calculate_origin_x(next_char) {
            Ok(x) => x + metrics.xmin as isize,
            Err(e) => return Some(Err(e))
        };

        let glyph_y = match self.calculate_baseline(&metrics) {
            Ok(b) => b - metrics.ymin as isize - metrics.height as isize,
            Err(e) => return Some(Err(e))
        };

        let next_origin = match self.settings.layout.direction {
            LayoutDirection::LeftToRight => match self.settings.layout.glyph_spacing {
                SpacingMode::Scale(scale) => glyph_x + (scale * metrics.advance_width.ceil()) as isize,
                SpacingMode::Constant(spacing) => glyph_x + spacing as isize,
            },
            LayoutDirection::TopToBottom => match self.settings.layout.glyph_spacing {
                SpacingMode::Scale(scale) => glyph_y + (scale * (metrics.bounds.height + DEFAULT_VERTICAL_SPACING)) as isize,
                SpacingMode::Constant(spacing) => glyph_y + spacing as isize,
            },
        };

        self.prev_data = Some((next_char, next_origin));

        Some(Ok((next_char, glyph_x, glyph_y)))
    }
}

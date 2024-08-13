use std::{iter::Rev, str::{Chars, Split}};
use fontdue::Metrics;
use thiserror::Error;
use crate::PixelChannel;
use super::TextSettings;

pub const DEFAULT_VERTICAL_SPACING: f32 = 10.0;

#[derive(Debug, Error)]
pub enum LayoutError {
    #[error("Font doesn't have line spacing. Use constant line spacing, or another font.")]
    MissingLineSpacing
}

// TODO: Implement alignment for `LayoutDirection::TopToBottom`
#[derive(PartialEq, Clone, Copy)]
pub enum LayoutAlign {
    Start,
    End
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
    pub align: LayoutAlign,
    pub line_spacing: SpacingMode,
    pub glyph_spacing: SpacingMode,
    pub use_kern: bool
}

impl Default for TextLayout {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::LeftToRight,
            align: LayoutAlign::Start,
            line_spacing: SpacingMode::Scale(1.0),
            glyph_spacing: SpacingMode::Scale(1.0),
            use_kern: true
        }
    }
}
pub struct LayoutIter<'a, T: PixelChannel> {
    settings: &'a TextSettings<T>,
    lines: Split<'a, char>,
    current_row_text: either::Either<Rev<Chars<'a>>, Chars<'a>>,

    // Previous char, x/y (depending on direction) coordinate of the next origin position
    prev_data: Option<(char, isize)>,

    row: usize
}

impl<'a, T: PixelChannel> LayoutIter<'a, T> {
    pub fn new(settings: &'a TextSettings<T>) -> Self {
        let mut lines = settings.text.split('\n');
        // Will never panic as `Split` always emits at least one item.
        let current_row_text = lines.next().unwrap().chars();
        let either_iters = Self::either_iter_from_chars(settings.layout.align, current_row_text);
        Self { lines, current_row_text: either_iters, prev_data: None, settings, row: 0 }
    }

    fn either_iter_from_chars(align: LayoutAlign, chars: Chars<'a>) -> either::Either<Rev<Chars<'a>>, Chars<'a>> {
        match align {
            LayoutAlign::Start => either::Either::Right(chars),
            LayoutAlign::End => either::Either::Left(chars.rev())
        }
    }

    /// Only used for left to right layouts. Calculate the origin for `next_char` using scaled kerning values.
    fn calculate_kerned_origin(&self, origin: isize, prev_char: char, next_char: char) -> isize {
        // If alignment is `LayoutAlign::End`, then `prev_char` is on the right, and `next_char` is on the left
        // The kern must be negated as it is moving the left character in the opposite direction,
        // instead of moving the right character
        let kern = match self.settings.layout.align {
            LayoutAlign::Start => self.settings.font.horizontal_kern(prev_char, next_char, self.settings.size).unwrap_or(0.0),
            LayoutAlign::End => -self.settings.font.horizontal_kern(next_char, prev_char, self.settings.size).unwrap_or(0.0)
        };

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
                    Some(line_metrics) => {
                        let first_line_height = line_metrics.ascent - line_metrics.descent;
                        let other_line_height = line_metrics.new_line_size * self.row as f32 * scale;
                        Ok((first_line_height + other_line_height) as isize)
                    },
                    None => Err(LayoutError::MissingLineSpacing)
                }
            },
            LayoutDirection::TopToBottom => match self.prev_data {
                Some((_prev_char, next_origin_y)) => Ok(next_origin_y),
                // Baseline of first character in a column
                None => Ok(metrics.height as isize + metrics.ymin as isize)
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
        let next_char = loop {
            match self.current_row_text.next() {
                Some(next_char) => {
                    break next_char;
                },
                None => {
                    self.current_row_text = Self::either_iter_from_chars(self.settings.layout.align, self.lines.next()?.chars());
                    self.row += 1;
                    self.prev_data = None;
                }
            }
        };

        let metrics = self.settings.font.metrics(next_char, self.settings.size);

        // Glyph x is the coordinate that the rasterized glyph should be drawn at.
        // It is an offset from the origin by `metrics.xmin`.
        let unshifted_glyph_x = match self.calculate_origin_x(next_char) {
            Ok(x) => x + metrics.xmin as isize,
            Err(e) => return Some(Err(e))
        };

        let baseline = match self.calculate_baseline(&metrics) {
            Ok(b) => b,
            Err(e) => return Some(Err(e))
        };

        let glyph_y = baseline - metrics.ymin as isize - metrics.height as isize;

        let direction_negation = if matches!(self.settings.layout.align, LayoutAlign::Start) { 1.0 } else { -1.0 };

        let shifted_glyph_origin = match self.settings.layout.direction {
            LayoutDirection::LeftToRight => match self.settings.layout.glyph_spacing {
                SpacingMode::Scale(scale) => unshifted_glyph_x + (scale * metrics.advance_width.ceil() * direction_negation) as isize,
                SpacingMode::Constant(spacing) => unshifted_glyph_x + (direction_negation*spacing) as isize,
            },
            LayoutDirection::TopToBottom => match self.settings.layout.glyph_spacing {
                SpacingMode::Scale(scale) => glyph_y + (scale * (metrics.height as f32 + DEFAULT_VERTICAL_SPACING)) as isize,
                SpacingMode::Constant(spacing) => baseline + spacing as isize,
            }
        };

        self.prev_data = Some((next_char, shifted_glyph_origin));

        Some(Ok((next_char, if matches!(self.settings.layout.align, LayoutAlign::Start) { unshifted_glyph_x } else { shifted_glyph_origin }, glyph_y)))
    }
}

use super::LayerFilter;
use crate::{pixel::ImagePixels, parser::ParsedArgs};

use std::collections::HashMap;

pub struct FlipFilter {
    args: HashMap<String, ParsedArgs>,
}

impl LayerFilter for FlipFilter {
    fn process(&self, pixels: &mut ImagePixels) {
        let direction = self.args.get("direction")
            .and_then(|x| Some(
                x.as_str().expect("Expected string (horizontal/vertical/both) for flip direction.")
            ));
        
        match direction {
            Some("vertical") => {
                // flip in place
                // move from top and bottom to center, swaping each row of pixels

                let split_point = (pixels.height()/2)*pixels.width();
                let odd_height = pixels.height() % 2 == 1;
                let width = pixels.width();
                let (top_half, bottom_half) = pixels.get_pixels_mut().split_at_mut(split_point);

                let bottom_half_adjusted;
                if odd_height {
                    bottom_half_adjusted = &mut bottom_half[width..];
                } else {
                    bottom_half_adjusted = bottom_half;
                }

                let top_rows = top_half.chunks_mut(width);
                let bottom_rows = bottom_half_adjusted.rchunks_mut(width);

                top_rows.zip(bottom_rows).for_each(|(top, bottom)| {
                    top.swap_with_slice(bottom)
                });
            },
            Some("horizontal") => {
                // reverse every row of pixels
                let width = pixels.width();
                pixels.get_pixels_mut().chunks_mut(width).for_each(|row| row.reverse());
            },
            Some("both") => {
                pixels.get_pixels_mut().reverse();
            },
            None => {
                // both as default direction
                pixels.get_pixels_mut().reverse();
            },
            Some(_) => panic!("Invalid flip direction (horizontal/vertical/both).")
        }
    }

    fn new_with_args(args: HashMap<String, ParsedArgs>) -> Self where Self: Sized {
        let args_string = HashMap::from_iter(
            args.into_iter()
        );
        Self { args: args_string }
    }
}

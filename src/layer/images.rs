use super::Layer;
use crate::{colors::*, pixel::ImagePixels, parser::ParsedArgs};

use image::{open, imageops};

pub struct Image {
    pixels: ImagePixels,
    position: (i32, i32)
}

impl Layer for Image {
    fn new_layer(args: std::collections::HashMap<String, ParsedArgs>) -> Self {
        let path = args.get("path")
            .expect("No path specified.")
            .as_str()
            .expect("Expected string for path.");
        let mut image = open(path).expect("Couldn't open image.");

        let scale = args.get("scale")
            .unwrap_or(&ParsedArgs::Float(1.0))
            .as_float()
            .expect("Expected a number for scale.");

        let filtertype_arg = args.get("scale-filter")
            .and_then(|x| Some(x.as_str().expect("Expected string for scale-filter.")))
            .and_then(|x| {
                Some(match x {
                    "cubic" => imageops::CatmullRom,
                    "gaussian" => imageops::Gaussian,
                    "nearest" => imageops::Nearest,
                    "triangle" => imageops::Triangle,
                    "lanczos" => imageops::Lanczos3,
                    _ => panic!("Invalid filter-type.")
                })
            })
            .unwrap_or(imageops::Lanczos3);

        let nwidth = ((image.width() as f64) * scale) as u32;
        let nheight = ((image.height() as f64) * scale) as u32;

        image = image.resize(nwidth, nheight, filtertype_arg);
        
        let width = image.width();
        let height = image.height();

        let rgba_img = image.to_rgba32f();
        let raw = rgba_img.into_raw();

        let pixels = raw.chunks(4).map(|p| 
            RGBAColor(
                (p.get(0).unwrap().clone() * 255.0) as f64,
                (p.get(1).unwrap().clone() * 255.0) as f64,
                (p.get(2).unwrap().clone() * 255.0) as f64,
                (p.get(3).unwrap().clone() * 255.0) as f64
            )
        );

        let mut position = args.get("position")
            .unwrap_or(&ParsedArgs::Coord2D((0, 0)))
            .as_coord_2d()
            .expect("Expected a 2D coordinate for position.");

        let center = args.get("center")
            .unwrap_or(&ParsedArgs::Boolean(false))
            .as_bool()
            .expect("Expected bool for 'center'.");

        if center {
            position = (position.0 - (width as i32)/2, position.1 - (height as i32)/2);
        }
        let image_pixels = ImagePixels::from_pixels(width as usize, pixels.collect());
        Self { pixels: image_pixels, position }
    }

    fn get_image(&mut self) -> &mut ImagePixels {
        &mut self.pixels
    }

    fn pixel_at(&self, x: u32, y: u32) -> Option<RGBAColor> {
        let x = x as i32;
        let y = y as i32;

        if (x < self.position.0) || (y < self.position.1) {
            return None
        }

        if (x >= self.position.0 + (self.pixels.width() as i32)) || (y >= self.position.1 + (self.pixels.height() as i32)) {
            return None
        }

        // in bounds
        let rel_x = x - self.position.0;
        let rel_y = y - self.position.1;

        // impossible for rel_x or rel_y to be negative, as x > x position, and y > y position
        self.pixels.get_pixel_at(rel_x as usize, rel_y as usize).cloned()
    }
}

use super::Layer;
use crate::{
    pixel::ImagePixels,
    parser::ParsedArgs,
    colors::RGBAColor
};
use std::collections::HashMap;

use font_loader::system_fonts;
use fontdue::{
    Font,
    FontSettings,
    layout::{
        Layout,
        CoordinateSystem,
        LayoutSettings,
        HorizontalAlign,
        VerticalAlign,
        WrapStyle,
        TextStyle
    }
};

fn rasterize_layout(layout: Layout, font: Font) -> ImagePixels {
    let size = layout.height();
    let glyphs = layout.glyphs();

    for glyph in glyphs {
        let (metrics, char_raster) = font.rasterize_config(glyph.key);
        let pixels = char_raster.iter().map(|x| {
            let float = *x as f64;
            RGBAColor (float, float, float, float*255.0)
        });
        let width = metrics.width;

        return ImagePixels::from_pixels(width as u32, pixels.collect());

        println!("{:?}", char_raster);
    }

    ImagePixels::from_pixels(0, vec![])
}

pub struct Text {
    pixels: ImagePixels
}

macro_rules! font_property_bool {
    ($args:ident, $name:literal) => {
        $args.get($name)
            .map_or(
                false, 
                |x| x.as_bool().expect(concat!("Expected a bool for ", $name))
            )
    };
}

impl Layer for Text {
    fn new_layer(
        args: HashMap<String, ParsedArgs>
    ) -> Self
        where Self: Sized {
        let text = args.get("text")
            .expect("Expected a 'text' for text.")
            .as_str()
            .expect("Expected a string for text");

        let font_name = args.get("font")
            .expect("Expected a font for text.")
            .as_str()
            .expect("Expected a string for font.");

        let font_bytes = {
            let bold = font_property_bool!(args, "bold");
            let italic = font_property_bool!(args, "italic");
            let oblique = font_property_bool!(args, "oblique");
            let monospace = font_property_bool!(args, "monospace");

            let mut font_properties = system_fonts::FontPropertyBuilder::new().family(font_name);
            if bold { font_properties = font_properties.bold() }
            if italic { font_properties = font_properties.italic() }
            if oblique { font_properties = font_properties.oblique() }
            if monospace { font_properties = font_properties.monospace() }

            let font_properties_built = font_properties.build();
            let (bytes, _) = system_fonts::get(&font_properties_built).expect("Font couldn't be loaded.");
            bytes
        };

        let font_size = args.get("size").map_or(40.0, |x| x.as_float().expect("Expected size to be a number (text).") as f32);
        let position = args.get("position")
            .unwrap_or(&ParsedArgs::Coord2D((0, 0)))
            .as_coord_2d()
            .expect("Expected a 2D coordinate for position.");
        let max_width = args.get("max-width")
            .map(|x| x.as_float().expect("Expected float for max-width.") as f32);

        let font_settings = FontSettings { collection_index: 0, scale: font_size };
        let font = Font::from_bytes(font_bytes, font_settings)
            .map_err(|err| panic!("Error creating font ({})", err))
            .unwrap();

        let mut layout = Layout::new(CoordinateSystem::PositiveYUp);
        let layout_settings = LayoutSettings {
            x: position.0 as f32,
            y: position.1 as f32,
            max_width,
            max_height: None,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Top,
            line_height: 1.0,
            wrap_style: WrapStyle::Word,
            wrap_hard_breaks: true,
        };
        layout.reset(&layout_settings);
        layout.append(&[&font], &TextStyle::new(text, font_size, 0));
        let rendered = rasterize_layout(layout, font);
        
        Self { pixels: rendered }
    }

    fn get_image(&mut self) -> &mut ImagePixels {
        &mut self.pixels
    }


    fn pixel_at(&self, x: u32, y: u32) -> Option<RGBAColor> {
        self.pixels.get_pixel_at(x, y).cloned()
    }
}

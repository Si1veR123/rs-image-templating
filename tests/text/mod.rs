
pub mod raster_text;
pub mod glyph_layout;

use fontdue::Font;

// TODO: include local font
static FONT_BYTES: &[u8] = include_bytes!(r"C:/Windows/Fonts/Calibri.ttf") as &[u8];

fn get_font() -> Font{
    Font::from_bytes(FONT_BYTES, fontdue::FontSettings { collection_index: 0, scale: 30.0, load_substitutions: true  }).unwrap()
}

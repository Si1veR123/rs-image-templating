
use image_template::{layers::text::{layout::{LayoutIter, TextLayout}, TextSettings}, pixels::pixel::AlphaPixel};

use crate::text::get_font;


#[test]
fn layout_basic() {
    let correct_layout = [
        ('T', 0, 11), ('h', 17, 9), ('e', 34, 15),
        (' ', 49, 30), ('q', 57, 15), ('u', 75, 15),
        ('i', 92, 10), ('c', 100, 15), ('k', 115, 9),
        (' ', 129, 30), ('b', 138, 9), ('r', 156, 15),
        ('o', 168, 15), ('w', 184, 15), ('n', 208, 15),
        (' ', 224, 30), ('f', 231, 9), ('o', 242, 15),
        ('x', 258, 15), (' ', 271, 30), ('j', 277, 10),
        ('u', 287, 15), ('m', 305, 15), ('p', 331, 15),
        ('s', 348, 15), (' ', 360, 30), ('o', 368, 15),
        ('v', 384, 15), ('e', 399, 15), ('r', 416, 15),
        (' ', 427, 30), ('a', 435, 15), (' ', 450, 30),
        ('l', 459, 9), ('a', 467, 15), ('z', 483, 16),
        ('y', 495, 15), (' ', 509, 30), ('d', 517, 9),
        ('o', 534, 15), ('g', 550, 15), ('.', 567, 26),
        ('S', 1, 46), ('p', 17, 51), ('h', 35, 45),
        ('i', 52, 46), ('n', 61, 51), ('x', 77, 51),
        (' ', 90, 66), ('o', 98, 51), ('f', 114, 45),
        (' ', 124, 66), ('b', 133, 45), ('l', 151, 45),
        ('a', 159, 51), ('c', 175, 51), ('k', 190, 45),
        (' ', 204, 66), ('q', 212, 51), ('u', 230, 51),
        ('a', 247, 51), ('r', 264, 51), ('t', 275, 48),
        ('z', 287, 52), (',', 299, 62), (' ', 307, 66),
        ('j', 313, 46), ('u', 323, 51), ('d', 340, 45),
        ('g', 356, 51), ('e', 372, 51), (' ', 387, 66),
        ('m', 396, 51), ('y', 420, 51), (' ', 434, 66),
        ('v', 441, 51), ('o', 456, 51), ('w', 472, 51),
        ('.', 495, 62)
    ];

    let settings = TextSettings {
        size: 30.0,
        fill: AlphaPixel::<u8>::default(),
        layout: TextLayout::default(),
        text: String::from("The quick brown fox jumps over a lazy dog.\nSphinx of black quartz, judge my vow."),
        font: get_font()
    };

    let mut count = 0;
    for position_res in LayoutIter::new(&settings) {
        match position_res {
            Ok(position) => {
                assert!(correct_layout.contains(&position));
                count += 1;
            },
            Err(e) => panic!("Error in text layout: {:?}", e)
        }
    }
    assert_eq!(count, correct_layout.len());
}

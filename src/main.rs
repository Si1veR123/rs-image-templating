use std::fmt::Write;
use chrono::NaiveDate;
use image::GenericImageView;
use image_template::layers::image::ImageLayer;
use image_template::layers::shapes::RectangleLayer;
use image_template::layers::text::layout::{LayoutAlign, SpacingMode, TextLayout};
use image_template::layers::text::{TextLayer, TextSettings};
use image_template::layers::Layer;
use image_template::pixels::image::Image;
use image_template::pixels::pixel::AlphaPixel;
use image_template::canvas::Canvas;
use image_template::rect::Rect;
use image_template::rgba;

static FONT_BYTES: &[u8] = include_bytes!(r"../Bayon.ttf") as &[u8];
static ALBUM_COVER: &[u8] = include_bytes!("../kny_cover.png") as &[u8];

struct AlbumDetails {
    title: String,
    artist: String,
    // should be 1952x1952
    // TODO: Add resizing
    cover: Image<u8>,
    tracklist: Vec<String>,
    release_date: chrono::NaiveDate,
    genre: String,
    run_time: chrono::Duration,
    theme_colors: Vec<AlphaPixel<u8>>,
    background_color: AlphaPixel<u8>
}

fn create_album_poster(details: AlbumDetails) -> Image<u8> {
    let default_text_layout = TextLayout::default();
    let close_line_space_layout = TextLayout { line_spacing: SpacingMode::Scale(0.7),..default_text_layout.clone() };
    let close_line_space_right_layout = TextLayout { align: LayoutAlign::End, use_kern: false, ..close_line_space_layout.clone() };

    let font = fontdue::Font::from_bytes(FONT_BYTES, fontdue::FontSettings { collection_index: 0, scale: 200.0, load_substitutions: true  }).unwrap();

    let mut canvas = Canvas::from_dimensions(2400, 3600);
    canvas.background = details.background_color;

    let album_cover_layer = ImageLayer { filters: vec![], im: details.cover, x: 224, y: 224  };
    let title_layer = TextLayer::new(
        TextSettings {
            size: 175.0,
            fill: AlphaPixel::black(),
            layout: default_text_layout.clone(),
            text: details.title,
            font: font.clone(),
        },
        217,
        2050
    ).unwrap();
    let artist_layer = TextLayer::new(
        TextSettings {
            size: 120.0,
            fill: AlphaPixel::black(),
            layout: default_text_layout.clone(),
            text: details.artist,
            font: font.clone(),
        },
        219,
        2285
    ).unwrap();

    let mut column_start = 0;
    for (col, tracks) in details.tracklist[0..18.min(details.tracklist.len())].chunks(9).enumerate() {
        let mut text = String::new();
        let mut line_buffer = String::new();
        for (index, track) in tracks.iter().enumerate() {
            write!(&mut line_buffer, "{}. {track}\n", index+col*9+1).unwrap();
            text.push_str(&line_buffer);
            line_buffer.clear();
        }

        let text_layer = TextLayer::new(
            TextSettings {
                size: 65.0,
                fill: AlphaPixel::black(),
                layout: close_line_space_layout.clone(),
                text,
                font: font.clone(),
            },
            219+column_start,
            2560
        ).unwrap();

        column_start = text_layer.get_rect().width + 50;
        canvas.add_layer(text_layer);
    }

    let mut release_date_layer = TextLayer::new(
        TextSettings {
            size: 65.0,
            fill: AlphaPixel::black(),
            layout: close_line_space_right_layout.clone(),
            text: details.release_date.format("Release Date\n%B %-d, %C%y").to_string(),
            font: font.clone(),
        },
        1887,
        2560
    ).unwrap();
    release_date_layer.x = 2175 - release_date_layer.get_rect().width;

    let mut genre_layer = TextLayer::new(
        TextSettings {
            size: 65.0,
            fill: AlphaPixel::black(),
            layout: close_line_space_right_layout.clone(),
            text: format!("Genre\n{}", details.genre),
            font: font.clone(),
        },
        1887,
        2825
    ).unwrap();
    genre_layer.x = 2175 - genre_layer.get_rect().width;

    let hours = details.run_time.num_hours();
    let minutes = details.run_time.num_minutes() % 60;
    let seconds = details.run_time.num_seconds() % 60;

    let text = if hours == 0 {
        if minutes == 0 {
            format!("Run Time\n{seconds}")
        } else {
            format!("Run Time\n{minutes}:{seconds}")
        }
    } else {
        format!("Run Time\n{hours}:{minutes}:{seconds}")
    };

    let mut runtime_layer = TextLayer::new(
        TextSettings {
            size: 65.0,
            fill: AlphaPixel::black(),
            layout: close_line_space_right_layout.clone(),
            text,
            font: font.clone(),
        },
        1887,
        3059
    ).unwrap();
    runtime_layer.x = 2175 - runtime_layer.get_rect().width;

    for (index, color) in details.theme_colors.into_iter().enumerate() {
        let rect = RectangleLayer::new(color, Rect { x: 2053 - index*154, y: 2213, width: 122, height: 122 });
        canvas.add_layer(rect);
    }

    canvas.add_layer(album_cover_layer);
    canvas.add_layer(release_date_layer);
    canvas.add_layer(genre_layer);
    canvas.add_layer(runtime_layer);
    canvas.add_layer(title_layer);
    canvas.add_layer(artist_layer);

    canvas.flatten()
}

fn main() {
    let cover = image::load_from_memory_with_format(ALBUM_COVER, image::ImageFormat::Png).unwrap();
    let cover_im = Image::from_pixels(
        cover.pixels().map(|p| AlphaPixel { r: p.2.0[0], g: p.2.0[1], b: p.2.0[2], a: p.2.0[3] }).collect(),
        1952
    ).unwrap();

    let album = AlbumDetails {
        title: String::from("Koi No Yokan"),
        artist: String::from("Deftones"),
        cover: cover_im,
        tracklist: vec![
            String::from("Swerve City"),
            String::from("Romantic Dreams"),
            String::from("Leathers"),
            String::from("Poltergeist"),
            String::from("Entombed"),
            String::from("Graphic Nature"),
            String::from("Tempest"),
            String::from("Gauze"),
            String::from("Rosemary"),
            String::from("Goon Squad"),
            String::from("What happened to you?")
        ],
        release_date: NaiveDate::from_ymd_opt(2012, 11, 9).unwrap(),
        genre: String::from("Alternative Metal"),
        run_time: chrono::Duration::seconds(3108),
        theme_colors: vec![
            AlphaPixel { r: 142, g: 168, b: 141, a: 255 },
            AlphaPixel { r: 181, g: 50, b: 37, a: 255 },
            AlphaPixel { r: 221, g: 68, b: 27, a: 255 },
            AlphaPixel { r: 68, g: 74, b: 40, a: 255 },
            AlphaPixel { r: 9, g: 3, b: 0, a: 255 },
        ],
        background_color: rgba!(212, 193, 177, 255),
    };

    let poster = create_album_poster(album);
    poster.save("cover.png", image::ImageFormat::Png).unwrap();
}

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use image_art::ansi_escape_sequences::{
    BackgroundColorRGB, TextColorRGB, CLEAR_SCREEN, CONTROL_SEQUENCE_INTRODUCER, RESET,
};
use image_art::env_vars::Config;
use image_art::utils;
use std::env;
use std::process;

// https://en.wikipedia.org/wiki/Block_Elements
const PIXEL_IDENTIFIER: &str = "\u{2588}";

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("ERROR: Parsing Env Args {err}");
        process::exit(1);
    });

    println!("{:?}", config);

    let img = image::open(&config.image_file_loc()).unwrap_or_else(|err| {
        eprintln!("ERROR: Opening Image File {err}");
        process::exit(1);
    });

    if config.file_write() {
        let filename = "resources/foo.txt";
        println!("Writing to file {filename}");
        utils::write_to_file(filename, img.pixels()).unwrap_or_else(|err| {
            eprintln!("ERROR: Write to file {filename} {err}");
            process::exit(1);
        });
    }

    let resized_img = resize_img(&img, config.resize_percentage());
    let (width, height) = resized_img.dimensions();

    println!("{CONTROL_SEQUENCE_INTRODUCER}{CLEAR_SCREEN}");
    let bg_color_rgb = BackgroundColorRGB(0, 0, 0);
    for y in 0..height {
        for x in 0..width {
            let rgba = resized_img.get_pixel(x, y).0;
            let text_color_rgb = TextColorRGB(rgba[0], rgba[1], rgba[2]);
            print!("{CONTROL_SEQUENCE_INTRODUCER}{text_color_rgb}{CONTROL_SEQUENCE_INTRODUCER}{bg_color_rgb}{PIXEL_IDENTIFIER}");
        }
        println!("{CONTROL_SEQUENCE_INTRODUCER}{RESET}");
    }
}

fn resize_img(img: &image::DynamicImage, percentage: f32) -> DynamicImage {
    println!("Resizing image by {percentage}");
    let (width, height) = img.dimensions();
    let nwidth = width as f32 * percentage;
    let nheight = height as f32 * percentage;
    img.resize(nwidth as u32, nheight as u32, FilterType::Gaussian)
}

#![allow(dead_code)]
#![allow(unused_imports)]

use image::imageops::FilterType;
use image_art::ansi_escape_sequences::{
    BackgroundColorRGB, TextColorRGB, ClearScreen, CONTROL_SEQUENCE_INTRODUCER, RESET,
};
use image_art::env_vars::Config;
use std::env;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{stdout, Error, Write};
use std::process;

use image::{GenericImageView, GenericImage, DynamicImage};

const PIXEL_IDENTIFIER: &str = "●";

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("ERROR: Parsing Env Args {err}");
        process::exit(1);
    });

    println!("{:?}", config);

    let img = image::open(&config.get_image_file_loc()).unwrap_or_else(|err| {
        eprintln!("ERROR: Opening Image File {err}");
        process::exit(1);
    });

    // let filename = "resources/foo.txt";
    // write_to_file(filename, img.pixels()).unwrap_or_else(|err| {
    //     eprintln!("ERROR: Write to file {filename} {err}");
    //     process::exit(1);
    // });

    // print!("\x1b[38;2;255;255;0mH\x1b[0;1;3;35me\x1b[95ml\x1b[42ml\x1b[0;41mo\x1b[0m")

    // let text_color_rgb = TextColorRGB(255, 0, 0);
    // let bg_color_rgb = BackgroundColorRGB(0, 255, 0);
    // println!("{CONTROL_SEQUENCE_INTRODUCER}{text_color_rgb}{CONTROL_SEQUENCE_INTRODUCER}{bg_color_rgb}JANIT{CONTROL_SEQUENCE_INTRODUCER}{RESET}");

    let resized_img = resize_img(&img, config.get_resize_percentage());
    let (width, height) = resized_img.dimensions();
    
    println!("{CONTROL_SEQUENCE_INTRODUCER}{ClearScreen}");
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
    println!("Resizing imgae by {}", percentage);
    let (width, height) = img.dimensions();
    let nwidth = width as f32 * percentage;
    let nheight = height as f32  * percentage;
    img.resize(nwidth as u32, nheight as u32, FilterType::Gaussian)
}

fn write_to_file<T>(filename: &str, contents: T) -> Result<(), Error>
where
    T: Iterator,
    <T as Iterator>::Item: Debug,
{
    match File::create(filename) {
        Ok(mut file) => {
            for content in contents {
                if let Err(e) = write!(file, "{:?}\n", content) {
                    return Err(e);
                }
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

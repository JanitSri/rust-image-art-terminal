mod ansi_escape_sequences;

use ansi_escape_sequences::{
    BackgroundColorRGB, TextColorRGB, CLEAR_SCREEN, CONTROL_SEQUENCE_INTRODUCER, RESET,
};
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageError};
use std::fmt::Error;

// https://en.wikipedia.org/wiki/Block_Elements
// https://en.wikipedia.org/wiki/Braille_Patterns
const PIXEL_IDENTIFIER: &str = "\u{28FF}";

#[derive(Debug)]
pub struct Image {
    img: DynamicImage,
}

impl Image {
    pub fn new(image_file_loc: &str) -> Result<Self, ImageError> {
        match image::open(image_file_loc) {
            Ok(img) => Ok(Image { img }),
            Err(e) => Err(e),
        }
    }

    pub fn image(&self) -> &DynamicImage {
        &self.img
    }

    pub fn resize_img(self, percentage: f32) -> Result<Image, Error> {
        println!("Resizing image by {percentage}");
        let (width, height) = self.img.dimensions();
        let nwidth = width as f32 * percentage;
        let nheight = height as f32 * percentage;
        Ok(Image {
            img: self
                .img
                .resize(nwidth as u32, nheight as u32, FilterType::Gaussian),
        })
    }

    pub fn display_img(&self) {
        println!("{CONTROL_SEQUENCE_INTRODUCER}{CLEAR_SCREEN}");
        let bg_color_rgb = BackgroundColorRGB(0, 0, 0);
        let (width, height) = self.img.dimensions();
        for y in 0..height {
            for x in 0..width {
                let rgba = self.img.get_pixel(x, y).0;
                let text_color_rgb = TextColorRGB(rgba[0], rgba[1], rgba[2]);
                print!("{CONTROL_SEQUENCE_INTRODUCER}{text_color_rgb}{CONTROL_SEQUENCE_INTRODUCER}{bg_color_rgb}{PIXEL_IDENTIFIER}");
            }
            println!("{CONTROL_SEQUENCE_INTRODUCER}{RESET}");
        }
    }

    // TODO: implement just using braille as pixel identifier and no color
}

mod utils;
mod env_vars;

use image::GenericImageView;
use image_art::Image;
use std::env;
use std::process;

fn main() {
    let config = env_vars::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("ERROR: Parsing Env Args {err}");
        process::exit(1);
    });

    println!("{:?}", config);

    let img = Image::new(&config.image_file_loc()).unwrap_or_else(|err| {
        eprintln!("ERROR: Opening Image File {err}");
        process::exit(1);
    });

    if config.file_write() {
        let filename = "resources/image_rgb.txt";
        println!("Writing to file {filename}");
        utils::write_to_file(filename, img.image().pixels()).unwrap_or_else(|err| {
            eprintln!("ERROR: Write to file {filename} {err}");
            process::exit(1);
        });
    }

    let resized_img = img.resize_img(config.resize_percentage()).expect("Did not expect an error here on resizing");
    resized_img.display_img();
}
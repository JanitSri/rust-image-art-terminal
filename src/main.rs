#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::Debug;
use std::{env, process, fs::File};
use std::io::{prelude::*, Error, stdout, self};

use image::{GenericImageView};

const CONTROL_SEQUENCE_INTRODUCER: &str = "\x1b[";
const RESET: &str = "0m";

fn main() {
    // let image_loc = get_image_loc_env_var(env::args()).unwrap_or_else(|err| {
    //     eprintln!("ERROR: Parsing Env Args {err}");
    //     process::exit(1);
    // });
    
    // let img = image::open(&image_loc).unwrap_or_else(|err| {
    //     eprintln!("ERROR: Opening Image File {err}");
    //     process::exit(1);
    // });

    // // println!("{:?}", img.pixels().collect::<Vec<(u32, u32, Rgba<u8>)>>());
    // let filename = "resources/foo.txt";
    // write_to_file(filename, img.pixels()).unwrap_or_else(|err| {
    //     eprintln!("ERROR: Write to file {filename} {err}");
    //     process::exit(1);
    // });

    // print!("\x1b[38;2;255;255;0mH\x1b[0;1;3;35me\x1b[95ml\x1b[42ml\x1b[0;41mo\x1b[0m")
    println!("{CONTROL_SEQUENCE_INTRODUCER}{}{CONTROL_SEQUENCE_INTRODUCER}{RESET}", set_text_color_rgb(0, 0, 255, "JANIT"));
}

fn set_text_color_rgb(r: u8, g: u8, b: u8, text: &str) -> String {
    format!("38;2;{r};{g};{b}m{text}")
}


fn get_image_loc_env_var(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();
    
    let image_loc = match args.next() {
        Some(arg) => arg,
        None => return Err("Missing argument with image location"),
    };

    Ok(image_loc)
}

fn write_to_file<T>(filename: &str, contents: T) -> Result<(), Error> 
    where 
        T: Iterator, 
        <T as Iterator>::Item: Debug
{
    match File::create(filename) {
        Ok(mut file) => {
            for content in contents {
                if let Err(e) = write!(file, "{:?}\n", content) {
                    return Err(e);
                }
            };
        }, 
        Err(e) => return Err(e),
    }
    
    Ok(())
}

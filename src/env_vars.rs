#[derive(Debug)]
pub struct Config {
    image_file_loc: String,
    resize_percentage: f32,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let image_loc = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing argument with image location"),
        };

        let resize = match args.next() {
            Some(arg) => arg.parse::<f32>().unwrap_or_else(|_| 1.0),
            None => 1.0,
        };

        Ok(Config {
            image_file_loc: image_loc,
            resize_percentage: resize,
        })
    }

    pub fn get_image_file_loc(&self) -> &String {
        &self.image_file_loc
    }

    pub fn get_resize_percentage(&self) -> f32 {
        self.resize_percentage
    }
}

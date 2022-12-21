use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, Write};

pub fn write_to_file<T>(filename: &str, contents: T) -> Result<(), Error>
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

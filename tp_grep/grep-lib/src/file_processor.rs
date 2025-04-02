use crate::errors::Error;
use crate::errors::Error::FileReadingError;
use std::fs::File;
use std::io::BufReader;

pub fn get_file_reader(path: String) -> Result<BufReader<File>, Error> {
    let file = File::open(path);
    if file.is_err() {
        return Err(FileReadingError);
    }
    Ok(BufReader::new(file.unwrap()))
}

use image::{io::Reader as ImageReader, ImageError};

pub fn process(path: String) -> Result<(), ImageError> {
    let img = match ImageReader::open("images/0001.png") {
        Ok(reader) => reader.decode(),
        Err(err) => return Err(ImageError::IoError(err)),
    }?;

    match img.save(path) {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}

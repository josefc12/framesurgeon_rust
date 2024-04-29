use image::io::Reader as ImageReader;

use crate::structs::DefaultState;

pub fn process(path: String, data: &mut DefaultState) {

    for pather in data.items.iter() {
        println!("{}",pather)
    }

    let img_attempt = ImageReader::open("images/0001.png");
    let img = img_attempt.unwrap().decode().unwrap();
    img.save(path).expect("image couldn't have been saved");

}

use image::io::Reader as ImageReader;
use image::RgbaImage;
use image::imageops::replace;


use crate::structs::DefaultState;

pub fn process(path: String, data: &mut DefaultState) {

    /*
    for pather in data.items.iter() {
        println!("{}",pather)
    }
    */
    //Get the infor from the first image
    let first_frame = ImageReader::open(data.items[0].clone());
    let first_frame = first_frame.unwrap().decode().unwrap();
    
    //Calculate the width and height of the final flipbook.
    let frame_count = data.items.len();
    let multiplicator = (frame_count as f64).sqrt().ceil() as u32;
    let image_scale = first_frame.height();
    let final_dimension = image_scale * multiplicator;

    //This currently sets the canvas to be a square.
    let mut img_canvas = RgbaImage::new(final_dimension, final_dimension);


    //Loop through the canvas and replace its pixels at particular location with the image loaded in first_frame.
    let mut frame_index: i32 = 0;
    for step_vertical in 0..=multiplicator-1 {

        let position_y: i64 = step_vertical as i64 * image_scale as i64;

        for step_horizontal in 0..=multiplicator-1 {
            
            //The image to paste onto the canvas
            if frame_index <= data.items.len() as i32 -1 {
                let current_frame = ImageReader::open(data.items[frame_index as usize].clone());
                let current_frame = current_frame.unwrap().decode().unwrap();
    
                let position_x: i64 = step_horizontal as i64 * image_scale as i64;
                replace(&mut img_canvas, &current_frame, position_x, position_y);
            }
            

            frame_index += 1;
        }

    }

    img_canvas.save(path).expect("image couldn't have been saved");

    /*
    let img_attempt = ImageReader::open("images/0001.png");
    let img = img_attempt.unwrap().decode().unwrap();
    img.save(path).expect("image couldn't have been saved");
    */

}

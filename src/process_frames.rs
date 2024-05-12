use image::io::Reader as ImageReader;
use image::RgbaImage;
use image::imageops::{replace, FilterType};


use crate::structs::DefaultState;

pub fn process(path: String, data: &mut DefaultState) {

    /*
    for pather in data.items.iter() {
        println!("{}",pather)
    }
    */
    //Get the infor from the first image
    //let first_frame = ImageReader::open(data.items[0].clone());
    //let first_frame = first_frame.unwrap().decode().unwrap();
    
    //Calculate the width and height of the final flipbook.
    //let frame_count = data.items.len();
    //let multiplicator = (frame_count as f64).sqrt().ceil() as u32;
    //let image_scale = first_frame.height();

    if data.fb_horizontal == 0 {data.fb_horizontal = data.fb_horizontal_default}
    if data.fb_vertical == 0 {data.fb_vertical = data.fb_vertical_default}
    if data.frame_size == 0 {data.frame_size = data.frame_size_default}

    let final_dimension = data.frame_size * data.fb_horizontal as u32;

    //This currently sets the canvas to be a square.
    let mut img_canvas = RgbaImage::new(final_dimension, final_dimension);


    //Loop through the canvas and replace its pixels at particular location with the image loaded in first_frame.
    let mut frame_index: i32 = 0;
    for step_vertical in 0..=data.fb_vertical-1 {

        let position_y: i64 = step_vertical as i64 * data.frame_size as i64;

        for step_horizontal in 0..=data.fb_horizontal-1 {
            
            //The image to paste onto the canvas
            if frame_index <= data.items.len() as i32 -1 {
                let current_frame = ImageReader::open(data.items[frame_index as usize].clone());
                let mut current_frame = current_frame.unwrap().decode().unwrap();

                if current_frame.height() != data.frame_size || current_frame.width() != data.frame_size {
                    current_frame = current_frame.resize_exact(data.frame_size, data.frame_size, FilterType::Triangle);
                }
            
                let position_x: i64 = step_horizontal as i64 * data.frame_size as i64;
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

pub fn set_default_fb_dimensions(frame_count: i32, data: &mut DefaultState) {
    let multiplicator = (frame_count as f64).sqrt().ceil() as u32;
    data.fb_horizontal_default = multiplicator;
    data.fb_vertical_default = multiplicator;

    //Initialize the variable size by the default sizes
    data.fb_horizontal = data.fb_horizontal_default;
    data.fb_vertical = data.fb_vertical_default;

}

pub fn set_default_frame_size(data: &mut DefaultState) {
    let first_frame = ImageReader::open(data.items[0].clone());
    let first_frame = first_frame.unwrap().decode().unwrap();

    data.frame_size_default = first_frame.height();
    data.frame_size = first_frame.height();

}


use std::path::PathBuf;
use std::fs;
use image::io::Reader as ImageReader;
use image::RgbaImage;
use image::imageops::{replace, FilterType};

use crate::structs::State;

pub fn process(path: PathBuf, in_frames_horizontal: Option<i32>, in_frames_vertical: Option<i32>, in_frame_width: Option<i32>, in_frame_height: Option<i32> ) {

    let frame_count = fs::read_dir(&path).unwrap().count() as i32;

    let frames_horizontal: i32 = match in_frames_horizontal {
        None => {
            get_default_dimension(&frame_count)
        },
        Some(value) => {
            value
        }
    };

    let frames_vertical: i32 = match in_frames_vertical {
        None => {
            get_default_dimension(&frame_count)
        },
        Some(value) => {
            value
        }
    };

    let frame_width: i32 = match in_frame_width {
        None => {
            //TODO: Calculate automatic horizontal frame amount
            256
        },
        Some(value) => {
            value
        }
    };

    let frame_height: i32 = match in_frame_height {
        None => {
            //TODO: Calculate automatic horizontal frame amount
            256
        },
        Some(value) => {
            value
        }
    };
    
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

}

fn get_default_dimension(frame_count: &i32) -> i32 {
    let frame_count = frame_count.clone() as f64;
    return frame_count.sqrt().ceil() as i32;
}

pub fn get_default_scale(model_frame: &) {
    let first_frame = ImageReader::open(data.items[0].clone());
    let first_frame = first_frame.unwrap().decode().unwrap();

    data.frame_size_default = first_frame.height();
    data.frame_size = first_frame.height();

}


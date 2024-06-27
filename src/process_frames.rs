
use std::cell::RefCell;
use std::path::PathBuf;
use std::fs;
use std::sync::Arc;
use image::io::Reader as ImageReader;
use image::{DynamicImage, RgbaImage};
use image::imageops::{replace, FilterType};

use crate::structs::State;

pub fn process(path: PathBuf, in_frames_horizontal: Option<u32>, in_frames_vertical: Option<u32>, in_frame_width: Option<u32>, in_frame_height: Option<u32> ) {

    let image_paths: Arc<RefCell<Vec<PathBuf>>> = Arc::new(RefCell::new(vec![]));
    for path in fs::read_dir(&path).unwrap(){
        image_paths.borrow_mut().push(path.unwrap().path())
    }

    let frame_count = fs::read_dir(&path).unwrap().count() as u32;
    println!("initial frame count: {}", frame_count);
    //Lazy management.
    let model_frame = ImageReader::open(&image_paths.borrow()[0]);
    let model_frame = model_frame.unwrap().decode().unwrap();

    let frames_horizontal: u32 = match in_frames_horizontal {
        None => {
            get_default_dimension(&frame_count)
        },
        Some(value) => {
            value
        }
    };

    let frames_vertical: u32 = match in_frames_vertical {
        None => {
            get_default_dimension(&frame_count)
        },
        Some(value) => {
            value
        }
    };

    let frame_width: u32 = match in_frame_width {
        None => {
            get_default_scale(&model_frame)
        },
        Some(value) => {
            value
        }
    };

    let frame_height: u32 = match in_frame_height {
        None => {
            get_default_scale(&model_frame)
        },
        Some(value) => {
            value
        }
    };
    
    //the scale of an image * the amount of images gives the length of one side
    let final_dimension: u32 = &frame_width * &frames_horizontal;
    println!("final dimension was: {}", final_dimension);
    //This currently sets the canvas to be a square.
    let mut img_canvas = RgbaImage::new(final_dimension, final_dimension);
    println!("amount of horizontal images: {}", frames_horizontal);
    println!("size of model image in pixels: {} x {}", model_frame.width(), model_frame.height());
    println!("canvas size in pixels: {} x {}", img_canvas.width(), img_canvas.height());

    
    //Loop through the canvas and replace its pixels at particular location with the image loaded in first_frame.
    let mut frame_index: u32 = 0;
    let mut position_y: u32;
    let mut position_x: u32;

    for step_vertical in 0..=&frames_vertical-1 {

        position_y = step_vertical * frame_width /*lazy */;

        for step_horizontal in 0..=&frames_horizontal-1 {
            
            //The image to paste onto the canvas
            if frame_index <= &frame_count - 1 {
                let current_frame  = ImageReader::open(&image_paths.borrow()[frame_index as usize].clone());
                let mut current_frame = current_frame.unwrap().decode().unwrap();

                if current_frame.height() != frame_width /*lazy */ || current_frame.width() != frame_width /*lazy */ {
                    current_frame = current_frame.resize_exact(frame_width /*lazy */, frame_width /*lazy */, FilterType::Triangle);
                }
            
                position_x = step_horizontal * frame_width /*lazy */;
                replace(&mut img_canvas, &current_frame, position_x as i64, position_y as i64);
            }
            
            frame_index += 1;
        }

    }

    let mut output_path:PathBuf = path.clone();
    output_path.push("/new_image.png");
    img_canvas.save(output_path).expect("image couldn't have been saved");
    

}

fn get_default_dimension(frame_count: &u32) -> u32 {
    let frame_count = frame_count.clone() as f64;
    println!("frame count as f64: {}", frame_count);
    println!("frame count ceiled: {}", frame_count.sqrt().ceil());
    println!("frame count ceiled as u32: {}", frame_count.sqrt().ceil() as u32);
    return frame_count.sqrt().ceil() as u32;
}

//Lazy ass return, lazy ass solution
fn get_default_scale(model_frame: &DynamicImage) -> u32 {
    return model_frame.width();
}


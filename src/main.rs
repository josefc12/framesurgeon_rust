use std::rc::Rc;

use clap::Parser;

mod structs;
mod process_frames;

fn main() {

    //Get user input from args
    let args = structs::CliInput::parse();

    //Process frames
    process_frames::process(args.input_path, args.horizontal_frame_amount, args.vertical_frame_amount, args.frame_width, args.frame_height);


}
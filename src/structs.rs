
use clap::Parser;

#[derive(Clone)]
pub struct State {

    /*
    pub name: String,
    pub items: Arc<Vec<String>>,
    pub frame_counter: i32,
    pub fb_horizontal_default: u32,
    pub fb_vertical_default: u32,
    pub frame_size_default: u32,
    pub fb_horizontal: u32,
    pub fb_vertical: u32,
    pub frame_size: u32,
    pub path: String,
    pub mode: ProcessMode,
    */
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessMode {
    Flipbook,
    Convert,
    AnimatedGif,
}

#[derive(Parser)]
pub struct CliInput {
    pub input_path: std::path::PathBuf,
    pub horizontal_frame_amount: Option<u32>,
    pub vertical_frame_amount: Option<u32>, 
    pub frame_width: Option<u32>,
    pub frame_height: Option<u32>
}

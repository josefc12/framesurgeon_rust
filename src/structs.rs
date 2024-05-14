use druid::{commands, AppDelegate, Command, Data, DelegateCtx, Env, Handled, Lens, Target};
use druid::text::{Formatter,Validation,Selection,ValidationError};
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use crate::process_frames;

#[derive(Clone, Data, Lens)]
pub struct DefaultState {
    pub name: String,
    pub items: Arc<Vec<String>>,
    pub frame_counter: i32,
    pub fb_horizontal_default: u32,
    pub fb_vertical_default: u32,
    pub frame_size_default: u32,
    pub fb_horizontal: u32,
    pub fb_vertical: u32,
    pub frame_size: u32,
    pub path: String
}

pub struct NumberFormatter;

impl Formatter<u32> for NumberFormatter {
    fn format(&self, value: &u32) -> String {
        value.to_string()
    }

    fn validate_partial_input(&self, input: &str, _sel: &Selection) -> Validation {
        if input.is_empty() || (input.len() <= 4 && input.chars().all(|c| c.is_digit(10))) {
            Validation::success()
        } else if input.len() > 4{
            Validation::failure(ValidationError::new(Error::new(ErrorKind::Other, "oh no!")))
        } else {
            Validation::failure(ValidationError::new(Error::new(ErrorKind::Other, "oh no!")))
        }
    }

    fn value(&self, input: &str) -> Result<u32, ValidationError> {
        if input.is_empty() {
            return Ok(0); // Return default value when input is empty
        }
        input.parse().map_err(|_| ValidationError::new(Error::new(ErrorKind::Other, "oh no!")))
    }

    fn format_for_editing(&self, value: &u32) -> String {
        value.to_string()
    }
}

pub struct Delegate;

impl AppDelegate<DefaultState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut DefaultState,
        _env: &Env,
    ) -> Handled {
        
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            process_frames::process(file_info.path().display().to_string(), data);
            data.path = file_info.path().display().to_string();
            return Handled::Yes;
        }
        

        if let Some(files) = cmd.get(commands::OPEN_FILES) {
            let mut items_vector: Vec<String> = vec![];
            for file in files.iter() {
                let path = file.path().display().to_string();
                println!("Files: {}", path);
                items_vector.push(path);
            }
            data.items = Arc::new(items_vector);
            data.frame_counter = data.items.len() as i32;

            //Calculate automatic default (square) flipbook dimensions:
            //TODO: more accurate, non square result
            process_frames::set_default_fb_dimensions(data.items.len() as i32, data);
            process_frames::set_default_frame_size(data);

            return Handled::Yes;

        }
        Handled::No
    }
}
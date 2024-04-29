use druid::{Data,Lens, AppDelegate, DelegateCtx, Target, Command, Env, Handled, commands,};
use std::sync::Arc;
use crate::process_frames;

#[derive(Clone, Data, Lens)]
pub struct DefaultState {
    pub name: String,
    pub items: Arc<Vec<String>>,
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

            return Handled::Yes;

        }
        Handled::No
    }
}
use druid::{Data,Lens, AppDelegate, DelegateCtx, Target, Command, Env, Handled, commands,};
use std::sync::{Arc};

#[derive(Clone, Data, Lens)]
pub struct DefaultState {
    pub name: String,
    pub items: Arc<Vec<String>>
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
        /*
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            if let Err(e) = std::fs::write(file_info.path(), &data[..]) {
                println!("Error writing file: {e}");
            }
            return Handled::Yes;
        }
        */

        if let Some(files) = cmd.get(commands::OPEN_FILES) {
            let mut items_vector: Vec<String> = vec![];
            for file in files.iter() {
                let path = file.path().display().to_string();
                println!("Files: {}", path);
                items_vector.push(path);
            }
            data.items = Arc::new(items_vector);

            /*
            match std::fs::read_to_string(file_info.path()) {
                Ok(_s) => {
                    println!("Files: {}", file_info.path().display().to_string());
                }
                Err(e) => {
                    println!("Error opening file: {e}");
                }
            }
            */
            return Handled::Yes;

        }
        Handled::No
    }
}
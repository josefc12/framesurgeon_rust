use druid::{AppLauncher, LocalizedString, WindowDesc};

mod ui;
mod structs;

const WINDOW_TITLE: LocalizedString<structs::DefaultState> = LocalizedString::new("Frame Surgeon 2");



fn main() {
    // describe the main window
    let main_window = WindowDesc::new(ui::build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((ui::ROOT_WIDTH, ui::ROOT_HEIGHT))
        .resizable(false);
    
    // create the initial app state
    let initial_state = structs::DefaultState {
        name: "World".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .configure_env(ui::init_fs_theme)
        .launch(initial_state)
        .expect("Failed to launch application");
}
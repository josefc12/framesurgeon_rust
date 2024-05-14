use druid::{AppLauncher, LocalizedString, WindowDesc};
use std::sync::Arc;
//use image::io::{Reader as ImageReader};

mod ui;
mod structs;
mod process_frames;

const WINDOW_TITLE: LocalizedString<structs::DefaultState> = LocalizedString::new("Frame Surgeon 2");



fn main() {
    //TEMP IMAGE FOR EXPORT TESTING
    //let img = ImageReader::open("images/0001.png")?.decode()?;
    //TEMP IMAGE FOR EXPORT TESTING


    // describe the main window
    let main_window = WindowDesc::new(ui::build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((ui::ROOT_WIDTH, ui::ROOT_HEIGHT))
        .resizable(true);
    
    let items_vector: Vec<String> = vec!["No frames loaded".into()];
    let items_arc = Arc::new(items_vector);

    // create the initial app state (another kind of data to pass into the UI)
    let initial_state = structs::DefaultState {
        name: "World".into(),
        items: items_arc,
        frame_counter: 0,
        fb_horizontal_default: 0,
        fb_vertical_default: 0,
        frame_size_default: 0,
        fb_horizontal: 0,
        fb_vertical: 0,
        frame_size: 0,
        path: "none".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .delegate(structs::Delegate)
        .configure_env(ui::init_fs_theme)
        .launch(initial_state)
        .expect("Failed to launch application");
}
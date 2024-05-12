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
    
    let items_vector: Vec<String> = vec![
        "item1".into(),
        "item2".into(),
        "item3".into(),
        "item4".into(),
        "item5".into(),
        "item6".into(),
        "item7".into(),
        "item8".into(),
        "item9".into(),
        "item10".into(),
        "item11".into(),
        "item12".into(),
        "item13".into(),
        "item14".into(),
        "item15".into(),
        "item16".into(),
        "item17".into(),
        "item18".into(),
        "item19".into(),
        "item20".into(),
        "item21".into(),
        "item22".into(),
        "item23".into(),
        "item24".into(),
        "item25".into(),
        "item26".into(),
        "item27".into(),
        "item28".into(),
    ];
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
        
    };

    // start the application
    AppLauncher::with_window(main_window)
        .delegate(structs::Delegate)
        .configure_env(ui::init_fs_theme)
        .launch(initial_state)
        .expect("Failed to launch application");
}
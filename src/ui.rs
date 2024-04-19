use druid::widget::{Align, Flex, Label, Container, SizedBox, Button, Padding, Scroll, List};
use druid::{Color, Env, Widget, WidgetExt, FileSpec,FileDialogOptions};
use druid::theme::{self};

use crate::structs::DefaultState;

pub const _VERTICAL_WIDGET_SPACING: f64 = 20.0;
pub const _TEXT_BOX_WIDTH: f64 = 200.0;
pub const ROOT_WIDTH: f64 = 646.0;
pub const ROOT_HEIGHT: f64 = 547.0;
pub const ROUNDED_RADIUS: f64 = 5.0;

//COLORS
const DARK_DEEP: druid::Color = Color::rgb8(15,15,15);
const DARK_NORMAL: druid::Color = Color::rgb8(34,34,34);
const FS_TEAL: druid::Color = Color::rgb8(45,158,119);
const TEXT_EMBEDDED: druid::Color = Color::rgb8(100,100,100);
const BLOCK_IN: druid::Color = Color::rgb8(24,24,24);

pub fn build_root_widget() -> impl Widget<DefaultState> {

    //THIS IS LITERALLY JUST COPIED FROM THE DOCS ----
    let rs = FileSpec::new("Rust source", &["rs"]);
    let txt = FileSpec::new("Text file", &["txt"]);
    let other = FileSpec::new("Bogus file", &["foo", "bar", "baz"]);
    let default_save_name = String::from("MyFile.txt");
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![rs, txt, other])
        .default_type(txt)
        .default_name(default_save_name)
        .name_label("Target")
        .title("Choose a target for this lovely file")
        .button_text("Export");
    let open_dialog_options = save_dialog_options
        .clone()
        .default_name("MySavedFile.txt")
        .name_label("Source")
        .title("Where did you put that file?")
        .button_text("Import");
    //THIS IS LITERALLY JUST COPIED FROM THE DOCS ----
    
    let button_load = Button::new("Load frames")
        .on_click(move |ctx, _, _| {
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
        });
    //let button_export = Button::new("Export");

    let header_flex = Flex::row().with_child(button_load);
    //let footer_flex = Align::right(Flex::row());
    
    // center the two widgets in the available space
    let header = Container::new(SizedBox::new(header_flex).expand_width().height(24.0).padding((4.0,0.0)))
        .background(DARK_DEEP);
    let footer = Container::new(SizedBox::empty().expand_width().height(24.0).padding((4.0,0.0)))
    .background(DARK_DEEP);
    

    //Scroller.
    let frame_scroller = 
        Scroll::new(
                List::new(|| {
                    Label::new(|item: &String, _env: &_| item.clone()).with_text_size(14.0)
                        .padding(5.0)
                        .expand_width()
                        .height(20.0)
                        .align_left()
                    }
                ).lens(DefaultState::items)
        )
        .vertical()
        .background(BLOCK_IN)
        .rounded(ROUNDED_RADIUS);

    //frame_list itself
    let frame_list = Padding::new(
        4.0,
        Container::new(Flex::column()
                .with_spacer(4.0)
                .with_child(Align::left(Label::new("Frames").with_text_color(TEXT_EMBEDDED).with_text_size(12.0)))
                .with_spacer(6.0)
                .with_flex_child(frame_scroller, 1.0)
                .with_spacer(12.0)
                .padding((4.0,0.0))
                )
            .background(DARK_NORMAL)
            .rounded(ROUNDED_RADIUS)
        );
    
    //Container that holds the list with the scroller list as its child
    let root_frame_list = SizedBox::new(frame_list);
    
    //Vertical flexbox for the settings
    let settings = Padding::new(
        4.0,
        Container::new(Flex::column()
            .with_child(Align::left(Label::new("Settings").with_text_color(TEXT_EMBEDDED).with_text_size(12.0)))
            .padding((4.0,0.0))
            )
        .background(DARK_NORMAL)
        .rounded(ROUNDED_RADIUS)
    );

    //Flexbox that holds the list and the settings
    let content_flex = 
        Padding::new(
            (4.0,0.0),
            Container::new(
                Flex::row()
                .with_flex_child(root_frame_list.expand_height(), 1.0)
                .with_child(settings.expand_height().fix_width(224.0).align_right()))
        .background(DARK_DEEP));
    
    //Container box that holds the list and the settings
    let root_content = Container::new(SizedBox::new(content_flex))
        .background(DARK_DEEP);
    
    //Rootest container. Holds the header, content and footer boxes
    let root_flex = Container::new(Flex::column()
        .must_fill_main_axis(true)
        .with_child(header.padding((4.0,4.0,4.0,0.0)))
        .with_flex_child(root_content.expand(), 1.0)
        .with_child(footer.padding((4.0,0.0,4.0,4.0)))
    ).background(DARK_DEEP);
    Align::centered(root_flex)
}

pub fn init_fs_theme(env: &mut Env, _state: &DefaultState) {
    env.set(theme::BUTTON_DARK, FS_TEAL);
    env.set(theme::BUTTON_LIGHT, FS_TEAL);
}
use druid::widget::{Align, Flex, Label, Container, SizedBox, Button, Padding};
use druid::{Env, Widget, WidgetExt, Color};
use druid::theme::{self};

use crate::structs::DefaultState;

pub const _VERTICAL_WIDGET_SPACING: f64 = 20.0;
pub const _TEXT_BOX_WIDTH: f64 = 200.0;
pub const ROOT_WIDTH: f64 = 646.0;
pub const ROOT_HEIGHT: f64 = 547.0;

//COLORS
const DARK_DEEP: druid::Color = Color::rgb8(15,15,15);
const DARK_NORMAL: druid::Color = Color::rgb8(34,34,34);
const FS_TEAL: druid::Color = Color::rgb8(45,158,119);
const TEXT_EMBEDDED: druid::Color = Color::rgb8(100,100,100);

pub fn build_root_widget() -> impl Widget<DefaultState> {

    let button_load = Button::new("Load frames");
    //let button_export = Button::new("Export");

    let header_flex = Flex::row().with_child(button_load);
    //let footer_flex = Align::right(Flex::row());
    
    // center the two widgets in the available space
    let header = Container::new(SizedBox::new(header_flex).expand_width().height(24.0).padding((4.0,0.0)))
        .background(DARK_DEEP);
    let footer = Container::new(SizedBox::empty().expand_width().height(24.0).padding((4.0,0.0)))
    .background(DARK_DEEP);
    
    //frame_list itself
    let frame_list = Padding::new(
        4.0,
        Container::new(Flex::column()
                .with_child(Align::left(Label::new("Frames").with_text_color(TEXT_EMBEDDED).with_text_size(12.0)))
                .padding((4.0,0.0))
                )
            .background(DARK_NORMAL)
            .rounded(5.0)
        );
    
    //Container that holds the list
    let root_frame_list = SizedBox::new(frame_list)
        .expand_height()
        .width(ROOT_WIDTH/2.0+50.0);
    
    //Vertical flexbox for the settings
    let settings = Padding::new(
        4.0,
        Container::new(Flex::column()
            .with_child(Align::left(Label::new("Settings").with_text_color(TEXT_EMBEDDED).with_text_size(12.0)))
            .padding((4.0,0.0))
            )
        .background(DARK_NORMAL)
        .rounded(5.0)
    );

    //Flexbox that holds the list and the settings
    let content_flex = 
        Padding::new(
            (4.0,0.0),
            Container::new(
                Flex::row()
                .with_child(root_frame_list)
                .with_flex_child(settings.expand(), 1.0))
        .background(DARK_DEEP));
    
    //Container box that holds the list and the settings
    let root_content = Container::new(SizedBox::new(content_flex))
        .background(DARK_DEEP);
    
    //Rootest container. Holds the header, content and footer boxes
    let root_flex = Container::new(Flex::column()
        .must_fill_main_axis(true)
        .with_child(header.padding((4.0,4.0,4.0,0.0)))
        .with_flex_child(root_content.expand_width().expand_height(), 1.0)
        .with_child(footer.padding((4.0,0.0,4.0,4.0)))
    ).background(DARK_DEEP);
    Align::centered(root_flex)
}

pub fn init_fs_theme(env: &mut Env, _state: &DefaultState) {
    env.set(theme::BUTTON_DARK, FS_TEAL);
    env.set(theme::BUTTON_LIGHT, FS_TEAL);
}
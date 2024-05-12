use druid::widget::{Align, Button, Container, Flex, Label, List, Padding, Scroll, SizedBox, TextBox, ValueTextBox};
use druid::{Color, Env, Widget, WidgetExt, FileSpec,FileDialogOptions};
use druid::theme::{self};

use crate::structs::DefaultState;
use crate::structs::NumberFormatter;

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

    let image_spec = FileSpec::new("Image files", &["bmp", "gif", "ico", "jpeg", "jpg", "png", "tga"]);
    let default_save_name = String::from("MyFlipbook.tga");

    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![image_spec])
        .default_type(image_spec)
        .default_name(default_save_name)
        .name_label("Target")
        .title("Export")
        .button_text("Export");

    let open_dialog_options = save_dialog_options
        .clone()
        .default_name("MySavedFlipbook.tga")
        .multi_selection()
        .name_label("Source")
        .title("Load frames")
        .button_text("Import");
    
    let button_load = Button::new("Load frames")
        .on_click(move |ctx, _, _| {
            let command = druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone());
            ctx.submit_command(command);
        });
    
    let button_export = Button::new("Export")
        .on_click(move |ctx, _, _| {
            let command = druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone());
            ctx.submit_command(command);
        });
    
    let setting_fb_reset = Button::from_label(Label::new("R").with_text_size(12.0)).fix_height(24.0).fix_width(28.0)
        .on_click(move |_ctx, data: &mut DefaultState, _| {
            data.fb_horizontal = data.fb_horizontal_default;
            data.fb_vertical = data.fb_vertical_default
        });

    let setting_frame_reset = Button::from_label(Label::new("R").with_text_size(12.0)).fix_height(24.0).fix_width(28.0)
    .on_click(move |_ctx, data: &mut DefaultState, _| {
        data.frame_size = data.frame_size_default;
    });
    let setting_frame_2 = Button::from_label(Label::new("/2").with_text_size(12.0)).fix_height(24.0).fix_width(28.0)
    .on_click(move |_ctx, data: &mut DefaultState, _| {
        data.frame_size = data.frame_size / 2;
    });
    let setting_frame_4 = Button::from_label(Label::new("/4").with_text_size(12.0)).fix_height(24.0).fix_width(28.0)
    .on_click(move |_ctx, data: &mut DefaultState, _| {
        data.frame_size = data.frame_size / 4;
    });
    let setting_frame_8 = Button::from_label(Label::new("/8").with_text_size(12.0)).fix_height(24.0).fix_width(28.0)
    .on_click(move |_ctx, data: &mut DefaultState, _| {
        data.frame_size = data.frame_size / 8;
    });
    
    let header_flex = Flex::row().with_child(button_load);
    let footer_flex = Align::right(Flex::row().with_child(button_export));
    
    // center the two widgets in the available space
    let header = Container::new(SizedBox::new(header_flex).expand_width().height(24.0).padding((4.0,0.0)))
        .background(DARK_DEEP);
    let footer = Container::new(SizedBox::new(footer_flex).expand_width().height(24.0).padding((4.0,0.0)))
    .background(DARK_DEEP);
    

    //Scroller.
    let frame_scroller = 
        Scroll::new(
            Padding::new(4.0,
                List::new(|| {
                    Label::new(|item: &String, _env: &_| item.clone()).with_text_size(14.0)
                        .expand_width()
                        .height(20.0)
                        .align_left()
                    }
                ).lens(DefaultState::items)
            )
        )
        .vertical()
        .background(BLOCK_IN)
        .rounded(ROUNDED_RADIUS);
    
    let frame_counter = Label::dynamic(|item: &i32, _env: &_| format!("Count: {}",item.clone().to_string()))
        .with_text_size(12.0)
        .with_text_color(TEXT_EMBEDDED)
        .lens(DefaultState::frame_counter)
        .align_left();

    //frame_list itself
    let frame_list = Padding::new(
        4.0,
        Container::new(Flex::column()
                .with_spacer(4.0)
                .with_child(Align::left(Label::new("Frames").with_text_color(TEXT_EMBEDDED).with_text_size(12.0)))
                .with_spacer(6.0)
                .with_flex_child(frame_scroller, 1.0)
                .with_spacer(6.0)
                .with_child(frame_counter)
                .with_spacer(12.0)
                .padding((4.0,0.0))
                )
            .background(DARK_NORMAL)
            .rounded(ROUNDED_RADIUS)
        );
    
    //Container that holds the list with the scroller list as its child
    let root_frame_list = SizedBox::new(frame_list);
    

    let setting_fb_size = Container::new(Flex::column()
        .with_child(Label::new("Flipbook size").with_text_color(TEXT_EMBEDDED).with_text_size(12.0).align_left())
        .with_spacer(6.0)
        .with_child(Padding::new(2.0,
            Flex::row()
            .with_child(
                Flex::row()
                    .with_child(ValueTextBox::new(TextBox::new().with_text_size(12.0),NumberFormatter)
                        .fix_width(42.0)
                        .fix_height(22.0)
                        .lens(DefaultState::fb_horizontal)
                        .on_click(move |ctx, _, _| {
                            ctx.request_focus();
                        }))
                    .with_child(Label::new("x").with_text_color(TEXT_EMBEDDED).with_text_size(12.0))
                    .with_child(ValueTextBox::new(TextBox::new().with_text_size(12.0),NumberFormatter)
                        .fix_width(42.0)
                        .fix_height(22.0)
                        .lens(DefaultState::fb_vertical)
                        .on_click(move |ctx, _, _| {
                            ctx.request_focus();
                        }))
                    .align_left()
                )
            .with_flex_child(
                Flex::row()
                    .with_child(setting_fb_reset)
                    .align_right()
                    ,1.0
                )
            
            )
        )
        .with_spacer(4.0)
    )
    .padding(4.0)
    .background(BLOCK_IN)
    .rounded(ROUNDED_RADIUS);


    let setting_frame_size = Container::new(Flex::column()
        .with_child(Label::new("Frame size").with_text_color(TEXT_EMBEDDED).with_text_size(12.0).align_left())
        .with_spacer(6.0)
        .with_child(Padding::new(2.0,
            Flex::row()
            .with_child(
                Flex::row()
                    .with_child(ValueTextBox::new(TextBox::new().with_text_size(12.0),NumberFormatter)
                        .fix_width(42.0)
                        .fix_height(22.0)
                        .lens(DefaultState::frame_size)
                        .on_click(move |ctx, _, _| {
                            ctx.request_focus();
                        }))
                    .with_child(Label::new("px").with_text_color(TEXT_EMBEDDED).with_text_size(12.0))
                    .align_left()
                )
            .with_flex_child(
                Flex::row()
                    .with_child(setting_frame_reset)
                    .with_child(setting_frame_2)
                    .with_child(setting_frame_4)
                    .with_child(setting_frame_8)
                    .align_right()
                    ,1.0
                )
            
            )
        )
        .with_spacer(4.0)
    )

    .padding(4.0)
    .background(BLOCK_IN)
    .rounded(ROUNDED_RADIUS);

    //Vertical flexbox for the settings
    let settings = Padding::new(
        4.0,
        Container::new(Flex::column()
            .with_spacer(4.0)
            .with_child(Align::left(Label::new("Settings").with_text_color(TEXT_EMBEDDED).with_text_size(12.0)))
            .with_spacer(6.0)
            .with_flex_child(setting_fb_size,1.0)
            .with_spacer(6.0)
            .with_flex_child(setting_frame_size,1.0)
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
    .on_click(move |ctx, _, _| {
        ctx.request_focus();
    })
}

pub fn init_fs_theme(env: &mut Env, _state: &DefaultState) {
    env.set(theme::BUTTON_DARK, FS_TEAL);
    env.set(theme::BUTTON_LIGHT, FS_TEAL);
}
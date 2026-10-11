mod gui;
use gui::{
    update, 
    view,
    the_boot
};

mod style;
use crate::style::the_theme;

mod core;

fn main() -> iced::Result {
    iced::application(
        the_boot, 
        update, 
        view
    )
    .theme(the_theme())
    .run()
}

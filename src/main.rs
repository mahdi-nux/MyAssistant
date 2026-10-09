mod gui;
use gui::{update, view};

mod style;
use crate::style::the_theme;

mod core;

fn main() -> iced::Result {
    iced::application(
        gui::State::default, 
        update, 
        view
    )
    .theme(the_theme())
    .run()
}

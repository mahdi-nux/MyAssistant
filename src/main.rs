mod gui;
use gui::{update, view};

mod style;
use crate::style::the_theme;

fn main() -> iced::Result {
    iced::application(
        gui::State::default, 
        update, 
        view
    )
    .theme(the_theme())
    .run()
}

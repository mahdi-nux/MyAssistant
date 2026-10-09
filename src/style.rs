use iced::{
    Theme,
    widget,
    Border,
    Color
};

pub fn the_theme() -> Theme {
    Theme::Moonfly
}

pub fn the_container() -> widget::container::Style {
    widget::container::Style {
        border: Border { 
            color: Color::from_rgb8(
                77, 
                107, 
                254
            ), 
            width: 2.0, 
            radius: 2.0.into() 
        },
        ..Default::default()
    }
}
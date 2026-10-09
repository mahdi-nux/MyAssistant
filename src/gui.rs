use iced::{
    Element, Length,
    widget::{self, text_editor},
};
use crate::style::the_container;

#[derive(Default)]
pub struct State {
    input: text_editor::Content,
    output: String,
}

#[derive(Clone)]
pub enum Message {
    Input(text_editor::Action),
    Send
}

pub fn view(state: &State) -> Element<'_, Message> {
    widget::column![
        widget::row![
            widget::container(
                widget::text_editor(&state.input)
                    .on_action(Message::Input)
                    .height(Length::FillPortion(1))
            )
            .width(Length::Fill),
            widget::container(
                widget::button("Send >")
                    .on_press(Message::Send)
            )
            .center_y(Length::Fill)
        ]
        .spacing(10),
        widget::container(
            widget::scrollable(
                widget::text(&state.output)
            )
        )
            .width(Length::Fill)
            .height(Length::FillPortion(2))
            .style(|_| the_container())
    ]
    .spacing(10)
    .padding(10)
    .into()
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::Input(action) => {
            state.input.perform(action);
        }
        Message::Send => {}
    }
}

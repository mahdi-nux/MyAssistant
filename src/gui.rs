use iced::{
    Element, 
    Length,
    widget::{self, text_editor},
    Task
};

use crate::style::the_container;
use crate::core::core;

#[derive(Default)]
pub struct State {
    input: text_editor::Content,
    output: String,
}

#[derive(Clone)]
pub enum Message {
    Input(text_editor::Action),
    Send,
    CoreFinished(String)
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
            .padding(10)
    ]
    .spacing(10)
    .padding(10)
    .into()
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Input(action) => {
            state.input.perform(action);
            Task::none()
        }
        Message::Send => {
            Task::perform(
                core(
                state.input.text().to_string()
                ), 
                Message::CoreFinished
            )
        }
        Message::CoreFinished(answer) => {
            state.output = answer;
            Task::none()
        }
    }
}

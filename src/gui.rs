use std::sync::{
    Arc,
    Mutex
};

use iced::{
    Element, 
    Length,
    widget::{self, text_editor},
    Task
};
use candle_transformers::models::qwen2::ModelForCausalLM;

use crate::style::the_container;
use crate::core::{
    build,
    generate
};

#[derive(Default)]
pub struct State {
    model: Option<Arc<Mutex<ModelForCausalLM>>>,
    input: text_editor::Content,
    output: String,
}

#[derive(Clone)]
pub enum Message {
    Builded(ModelForCausalLM),
    Input(text_editor::Action),
    Send,
    CoreFinished(String)
}

pub fn the_boot() -> (State, Task<Message>) {
    (
        State::default(),
        Task::perform(build(), Message::Builded)
    )
}

pub fn view(state: &State) -> Element<'_, Message> {
    widget::column![
        widget::row![
            widget::container(
                widget::text_editor(&state.input)
                    .on_action(Message::Input)
                    .height(Length::FillPortion(1))
                    .size(24)
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
                    .size(24)
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
        Message::Builded(model) => {
            state.model = Some(Arc::new(Mutex::new(model)));
            Task::none()
        }
        Message::Input(action) => {
            state.input.perform(action);
            Task::none()
        }
        Message::Send => {
            let Some(model) = state.model.as_ref().cloned()
            else {
                return Task::none();
            };

            Task::perform(
                generate(
                state.input.text().to_string(),
                model
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

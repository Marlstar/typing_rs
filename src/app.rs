use iced::{
    widget::{
        Button, button,
        Text, text,
        Row, Column
    },
    Background
};
use iced::Color as Colour;
use crate::ui::keyboard::KeyboardUI;

#[derive(Debug, Clone)]
pub enum Message {
    None
}

pub struct TypingApp {
    keyboard: KeyboardUI
}
impl Default for TypingApp {
    fn default() -> Self {
        return Self {
            keyboard: KeyboardUI::default()
        }
    }
}
impl TypingApp {
    pub fn run() -> iced::Result {
        iced::program(
            "Typing app",
            TypingApp::update,
            TypingApp::view
        )
            .window_size((1100.0, 800.0))
            .run()
    }
}

impl TypingApp {
    fn update(&mut self, message: Message) {

    }

    fn view(&self) -> Column<Message> {
        return Column::<Message>::new().push(text("Hello world?"));
        // return self.keyboard.build();
    }
}
use std::any::Any;
use std::io::Write;
use iced::{
    widget::{
        Button, button,
        text,
        Row, Column
    },
    Background, alignment
};
use iced::Color as Colour;
use crate::app::Message;

mod keyboard_consts {
    pub const KEY_WIDTH: f32 = 30.0;
    pub const KEY_HEIGHT: f32 = 30.0;

    pub mod key_colours {
        use iced::Color as Colour;
        pub const DEFAULT: Colour = Colour::from_rgb(0.8352941176, 0.8352941176, 0.8352941176); // Gray
    }
    pub mod text_colours {
        use iced::Color as Colour;
        pub const DEFAULT: Colour = Colour::from_rgb(0.1568627451, 0.1725490196, 0.2039215686); // Dark gray
    }
}


pub struct KeyboardUI {
    structure: KeyboardStructure
}
impl Default for KeyboardUI {
    fn default() -> Self {
        return Self {
            structure: Default::default()
        }
    }
}
impl KeyboardUI {
    pub fn build_(&self) -> Column<Message> {
        let mut out = Column::<Message>::new();

        let refs: [&[KeyboardButton]; 5] = [&self.structure.number_row, &self.structure.top_row, &self.structure.middle_row,
            &self.structure.bottom_row, &self.structure.space_row];

        let mut keyboard_col = Column::<Message>::new();
        for row_refs in refs {
            let mut row = Row::<Message>::new();
            for k in row_refs {
                row = row.push(k.build())
            }
            keyboard_col = keyboard_col.push(row);
            println!("  |> added row")
        }

        return out;
    }


    pub fn build(&self) -> Column<Message> {
        //TODO make actually build the full keyboard once I can get 1 key to work
        let button = KeyboardButton::new('A').build();
        return Column::new()
            .push(button);
    }
}

struct KeyboardStructure {
    pub number_row: [KeyboardButton; 14], // ~ 1 2 3 4 5 6 7 8 9 0 - = BACKSPACE
    pub top_row: [KeyboardButton; 14], // TAB Q W E R T Y U I O P [ ] \
    pub middle_row: [KeyboardButton; 13], // CAPS A S D F G H J K L ; ' ENTER
    pub bottom_row: [KeyboardButton; 12], // LSHIFT Z X C V B N M , . / RSHIFT
    pub space_row: [KeyboardButton; 10], // LCTRL fn WIN left_alt SPACE right_alt RCTRL LEFT UP/DOWN RIGHT
}
impl Default for KeyboardStructure {
    fn default() -> Self {
        return Self::build_qwerty();
    }
}

macro_rules! key {
    ($x: literal, $y: literal) => {
        KeyboardButton::with_extra_width($x, $y)
    };
    ($x: literal) => {
        KeyboardButton::new($x)
    };
}
macro_rules! keys {
    ($($item:expr),*) => {
        [
            $(key!($item),)*
        ]
    }
}
macro_rules! wide_keys {
    ($($x:expr, $y:literal,)*) => {
        [
            $(key!($x,$y),)*
        ]
    }
}
impl KeyboardStructure {
    fn build_qwerty() -> Self {
        type K = KeyboardButton;
        return Self {
            number_row: keys!("~", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "BACKSPACE"),
            top_row: keys!("TAB", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\"),
            middle_row: keys!("CAPS", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "\"", "ENTER"),
            bottom_row: keys!("SHIFT", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "SHIFT"),
            space_row: [ //keys!("CTRL", "fn", "WIN", "ALT", "SPACE", "ALT", "CTRL", "<", "^", ">")
                key!("CTRL"), key!("fn"), key!("WIN"), key!("ALT"), key!("SPACE", 30f32),
                key!("ALT"), key!("CTRL"), key!("<"), key!("^"), key!(">")
            ]
        }
    }
}

#[derive(Debug, Clone)]
struct KeyboardButton {
    extra_width: f32,
    text: String
}
impl Default for KeyboardButton {
    fn default() -> Self {
        return Self {
            extra_width: 0.0,
            text: "A".to_string()
        }
    }
}
impl KeyboardButton {
    pub fn new(content: impl Into<String>) -> Self {
        return Self {
            text: content.into(),
            ..Default::default()
        }
    }

    pub fn with_extra_width(content: impl Into<String>, x: f32) -> Self {
        let mut out = Self::new(content);
        out.set_extra_width(x);
        return out;
    }
    pub fn set_extra_width(&mut self, x: f32) {
        self.extra_width = x;
    }
}
impl KeyboardButton {
    pub fn build_coloured(&self, colour: ButtonColour) -> Button<Message> {
        // print!("{}| ", self.text); let _ = std::io::stdout().flush();

        let text = text(self.text.clone())
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center);

        let button: Button<Message> = button(text)
            .width(keyboard_consts::KEY_WIDTH + self.extra_width)
            .height(keyboard_consts::KEY_HEIGHT)
            .style(button_colour_style(colour));

        return button;
    }
    pub fn build(&self) -> Button<Message> {
        return self.build_coloured(ButtonColour::Standard);
    }
}

fn button_colour_style(colour: ButtonColour) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, _status: button::Status| -> button::Style {
        button::Style {
            background: Some(Background::Color(colour.colour())),
            text_color: Default::default(),
            border: Default::default(),
            shadow: Default::default(),
        }
    }
}

#[derive(Debug)]
enum ButtonColour {
    Standard,
    Index,
    Middle,
    Ring,
    Pinky,
    Other(Colour)
}
impl ButtonColour {
    pub fn colour(&self) -> Colour {
        return match self {
            Self::Standard => keyboard_consts::key_colours::DEFAULT,
            Self::Other(a) => a.clone(),
            _ => keyboard_consts::key_colours::DEFAULT
        }
    }
}
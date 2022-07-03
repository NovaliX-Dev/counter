#![windows_subsystem = "windows"]

use iced::{
    alignment::{Horizontal, Vertical},
    button,
    window::Settings as WSettings,
    Button, Column, Element, Length, Padding, Sandbox, Settings, Text,
};

// =============================================================================
// Macros
// =============================================================================

macro_rules! add_text {
    ($content: expr, $font_size: expr) => {
        Text::new($content)
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center)
            .size($font_size)
    };

    ($content: expr, $font_size: expr, ($w: expr, $h: expr)) => {
        add_text!($content, $font_size).width($w).height($h)
    };
}

macro_rules! add_btn {
    ($text: expr, ($w: expr, $h: expr), $state: expr, $message: expr, $disable_if: expr) => {{
        let mut btn = Button::new(
            &mut $state,
            add_text!($text, 30, (Length::Fill, Length::Fill)),
        )
            .width($w)
            .height($h);

        if !($disable_if) {
            btn = btn.on_press($message)
        }

        btn
    }};
}

macro_rules! compose {
    ($root: expr => ($($el: expr), *)) => {
        $root
            $(
                .push($el)
            )*
    };
}

// =============================================================================
// Messages
// =============================================================================

#[derive(Debug, Clone, Copy)]
enum Message {
    BtnIncrementPressed,
    BtnDecrementPressed,
}

// =============================================================================
// App
// =============================================================================

type VType = i8;

#[derive(Default)]
struct App {
    value: VType,

    increment_button: button::State,
    decrement_button: button::State,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App::default()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn view(&mut self) -> Element<Self::Message> {
        compose!(Column::new() => (
            add_btn!(
                "+",
                (Length::Fill, Length::FillPortion(2)),
                self.increment_button,
                Message::BtnIncrementPressed,
                self.value == VType::MAX
            ),
            add_text!(self.value.to_string(), 50, (Length::Fill, Length::FillPortion(3))),
            add_btn!(
                "-",
                (Length::Fill, Length::FillPortion(2)),
                self.decrement_button,
                Message::BtnDecrementPressed,
                self.value == VType::MIN
            )
        ))
            .padding(Padding::from(25))
            .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::BtnIncrementPressed => {
                if self.value < VType::MAX {
                    self.value += 1;
                }
            }
            Message::BtnDecrementPressed => {
                if self.value > VType::MIN {
                    self.value -= 1;
                }
            }
        }
    }
}

// =============================================================================
//
// =============================================================================

fn main() -> iced::Result {
    let initial_settings = Settings::<()> {
        window: WSettings {
            size: (450, 600),
            min_size: Some((300, 400)),

            ..Default::default()
        },

        ..Default::default()
    };

    App::run(initial_settings)
}


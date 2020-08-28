use iced::{
    button, text_input, Application, Button, Column, Command, Container, Element, Row, Settings,
    Text, TextInput,
};
use iced::{executor, Background, Color, HorizontalAlignment, Length};

use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    SpaceifierApp::run(Settings::default());
}

struct CopyButtonStyle;

impl button::StyleSheet for CopyButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            background: Some(Background::Color(Color {
                r: 0.11,
                g: 0.42,
                b: 0.87,
                a: 1.0,
            })),
            border_radius: 10,
            border_width: Default::default(),
            border_color: Color::BLACK,
            text_color: Color::WHITE,
        }
    }
}

// #[derive(Default)]
struct SpaceifierApp {
    text_input_state: text_input::State,
    text_input_value: String,
    text_output: String,
    number_input_state: text_input::State,
    number_input_value: String,
    copy_button_state: button::State,
    clipboard_context: ClipboardContext,
}

#[derive(Debug, Clone)]
pub enum Message {
    TextInputChanged(String),
    NumberInputChanged(String),
    CopyButtonPress,
}

impl Application for SpaceifierApp {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            SpaceifierApp {
                text_input_state: text_input::State::new(),
                text_input_value: String::new(),
                text_output: String::new(),
                number_input_state: text_input::State::new(),
                number_input_value: String::new(),
                copy_button_state: button::State::new(),
                clipboard_context: ClipboardContext::new().unwrap(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Spaceifier")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextInputChanged(new_value) => {
                self.text_input_value = new_value;
            }
            Message::NumberInputChanged(new_value) => {
                self.number_input_value = new_value;
            }
            Message::CopyButtonPress => {
                self.clipboard_context
                    .set_contents(self.text_output.clone())
                    .unwrap();
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let space_number: i32 = if !self.number_input_value.is_empty() {
            self.number_input_value
                .trim()
                .parse()
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    0
                })
        } else {
            0
        };
        self.text_output = spaceifier::spaceify(&self.text_input_value, space_number);

        // All main UI is held here
        Container::new(
            Column::new()
                .push(Text::new("Spaceifier").horizontal_alignment(HorizontalAlignment::Right))
                .push(
                    Row::new()
                        .push(
                            TextInput::new(
                                &mut self.text_input_state,
                                "Enter text here...",
                                self.text_input_value.as_str(),
                                Message::TextInputChanged,
                            )
                            .size(30)
                            .padding(10),
                        )
                        .push(
                            TextInput::new(
                                &mut self.number_input_state,
                                "Spaces",
                                self.number_input_value.as_str(),
                                Message::NumberInputChanged,
                            )
                            .size(30)
                            .padding(10)
                            .width(Length::Units(100))
                            .max_width(100),
                        )
                        .spacing(10),
                )
                .push(
                    Text::new(if self.text_input_value.is_empty() {
                        "Nothing to spaceify."
                    } else {
                        self.text_output.as_str()
                    })
                    .size(30),
                )
                .push(
                    Button::new(
                        &mut self.copy_button_state,
                        Text::new("Copy").size(20).color([1.0, 1.0, 1.0]),
                    )
                    .on_press(Message::CopyButtonPress)
                    .style(CopyButtonStyle)
                    .padding(5),
                )
                .padding(20)
                .spacing(20)
                .max_width(500),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

use iced::{Length, Row, Text, TextInput, text_input::State};

use crate::my_app::MyAppMessage;

#[derive(Clone)]
pub struct NumberInput {
    input_state: State,
    text_value: String,
    pub value: Option<f64>,
    message: fn(String) -> MyAppMessage,
}

impl NumberInput {
    pub fn new<'a>(message: fn(String) -> MyAppMessage) -> NumberInput {
        NumberInput {
            input_state: State::new(),
            text_value: String::from(""),
            value: None,
            message,
        }
    }

    pub fn view(&mut self) -> TextInput<MyAppMessage> {
        TextInput::new(
            &mut self.input_state,
            "1234",
            &self.text_value,
            self.message,
        )
        .size(20)
        .padding(5)
        .width(Length::Units(100))
    }

    pub fn update(&mut self, value: String) {
        if value.is_empty() {
            self.value = None;
            self.text_value = value;
            return;
        }
        if let Ok(num) = value.parse() {
            self.value = Some(num);
            self.text_value = value;
        }
    }
}

pub struct NumberInputRow {
    input: NumberInput,
    label: &'static str,
}

impl NumberInputRow {
    pub fn new(message: fn(String) -> MyAppMessage, label: &'static str) -> NumberInputRow {
        NumberInputRow {
            input: NumberInput::new(message),
            label,
        }
    }

    pub fn view(&mut self) -> Row<MyAppMessage> {
        Row::new()
            .push(Text::new(self.label).width(Length::Units(80)))
            .push(Row::new().push(self.input.view()))
            .padding(10)
    }

    pub fn update(&mut self, value: String) {
        self.input.update(value);
    }

    pub fn value(&self) -> Option<f64> {
        self.input.value
    }
}

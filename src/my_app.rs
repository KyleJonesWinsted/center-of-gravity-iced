use iced::{Align, Column, Element, Length, Row, Sandbox, Text};

use crate::number_input::NumberInputRow;

pub struct MyApp {
    top_input: NumberInputRow,
    base_input: NumberInputRow,
    height_input: NumberInputRow,
    answer: String,
}

#[derive(Debug, Clone)]
pub enum MyAppMessage {
    SetTop(String),
    SetBase(String),
    SetHeight(String),
}

macro_rules! number_inputs {
    ($view:ident) => {
        Column::new()
            .push($view.top_input.view())
            .push($view.base_input.view())
            .push($view.height_input.view())
            .padding(25)
    };
}

macro_rules! results {
    ($view:ident) => {
        Column::new()
            .push(Text::new("").height(Length::Units(20)))
            .push(Text::new("Center of gravity is: "))
            .push(Text::new(&$view.answer).size(35).height(Length::Units(35)))
            .push(Text::new("from the base"))
            .width(Length::FillPortion(1))
            .padding(25)
            .spacing(5)
            .align_items(Align::Center)
    };
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> MyApp {
        MyApp {
            top_input: NumberInputRow::new(MyAppMessage::SetTop, "Top: "),
            base_input: NumberInputRow::new(MyAppMessage::SetBase, "Base: "),
            height_input: NumberInputRow::new(MyAppMessage::SetHeight, "Height: "),
            answer: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("Calculate Center of Gravity")
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            MyAppMessage::SetTop(val) => self.top_input.update(val),
            MyAppMessage::SetBase(val) => self.base_input.update(val),
            MyAppMessage::SetHeight(val) => self.height_input.update(val),
        }
        self.calculate_answer();
    }

    fn view(&mut self) -> Element<Self::Message> {
        Row::new()
            .push(number_inputs!(self))
            .push(results!(self))
            .into()
    }
}

impl MyApp {
    fn calculate_answer(&mut self) {
        let top = self.top_input.value();
        let base = self.base_input.value();
        let height = self.height_input.value();
        let answer = match (top, base, height) {
            (Some(top), Some(base), _) if top + base == 0.0 => String::from("Divide by Zero"),
            (Some(top), Some(base), Some(height)) => {
                let height = height * 12.0;
                let center = (height * (base + (2.0 * top))) / (3.0 * (base + top)) / 12.0;
                Self::format_center_of_gravity(center)
            }
            _ => String::from(""),
        };
        self.answer = answer;
    }

    fn format_center_of_gravity(center: f64) -> String {
        let feet = center.floor();
        let inches = ((center - feet) * 12.0).floor();
        let feet_str = if feet == 1.0 { "foot" } else { "feet" };
        let inch_str = if inches == 1.0 { "inch" } else { "inches" };
        format!("{} {} {} {}", feet, feet_str, inches, inch_str)
    }
}

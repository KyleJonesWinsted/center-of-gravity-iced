mod my_app;
mod number_input;

use crate::my_app::MyApp;
use iced::Sandbox;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = iced::Settings {
        window: iced::window::Settings {
            size: (600, 200),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    };
    if let Err(e) = MyApp::run(settings) {
        println!("{}", e);
    }
    Ok(())
}


use iced::widget::{button, row, column, text};
use iced::{Length, Center, Element, window};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

pub struct State {
    current: String,
    err: Option<String>
}

impl Default for State {
    fn default() -> Self {
        State {
            current: get_current(Path::new("/proc/acpi/ibm/fan")),
            err: None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Off,
    Low,
    Medium,
    High,
    Max,
    Auto,
}

impl State {
    fn title(&self) -> String {
        match &self.err {
            Some(e) => e.clone(),
            None => "ThinkPad fan".to_string()
        }
    }

    fn update(&mut self, message: Message) {
        let path = Path::new("/proc/acpi/ibm/fan");
        let out = match message {
            Message::Off => "Off",
            Message::Low => "Low",
            Message::Medium => "Medium",
            Message::High => "High",
            Message::Max => "Max",
            Message::Auto => "Auto",
        };
        match echo(translate(out), path) {
            Ok(_) => {
                self.current = get_current(path);
                self.err = None;
            },
            Err(e) => {
                self.err = Some(format!("Can't write to file: {:?}", e));
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text(format!("level: {}", self.current))
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(Center)
            .size(50),
            row![
                button("Off").on_press(Message::Off),
                button("Low").on_press(Message::Low),
                button("Medium").on_press(Message::Medium),
                button("High").on_press(Message::High),
                button("Max").on_press(Message::Max),
                button("Auto").on_press(Message::Auto),
            ]
            .align_y(Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(15)
            .spacing(5)
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Center)
        .padding(20)
        .into()
    }
}

fn translate(a: &str) -> &str {
    match a {
        "Off" => "level 0",
        "Low" => "level 2",
        "Medium" => "level 4",
        "High" => "level 7",
        "Max" => "level disengaged",
        "Auto" => "level auto",
        _ => "level auto"
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn get_current(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s.lines()
        .find(|line| line.contains("level:"))
        .and_then(|line| line.split('\t').last())
        .unwrap_or("")
        .to_string(),
        Err(_) => String::new()
    }
}

pub fn main() -> iced::Result {
    let settings =
    window::Settings {
        size: iced::Size::new(415.0, 150.0),
        resizable: false,
        ..Default::default()
    };

    iced::application(State::title, State::update, State::view)
    .window(settings)
    .centered()
    .run()
}

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use iced::widget::{button, column, text, row};
use iced::{Alignment, Element, Sandbox, Settings};

struct State
{
    current: String,
    err: Option<String>
}

impl Sandbox for State
{
    type Message = Message;

    fn new() -> Self
    {
        let path: &Path = Path::new("/proc/acpi/ibm/fan");
        let c = get_current(path);
        State
        {
            current: c,
            err: None
        }
    }

    fn title(&self) -> String
    {
        match self.err.clone()
        {
            Some(e) => e,
            None => "ThinkPad fan".to_string()
        }
    }

    fn view(&self) -> Element<Message>
    {
        column![
            text(format!("level: {}",self.current.clone())).size(50),
            row![
                button("Off").on_press(Message::Off),
                button("Low").on_press(Message::Low),
                button("Medium").on_press(Message::Medium),
                button("High").on_press(Message::High),
                button("Max").on_press(Message::Max),
                button("Auto").on_press(Message::Auto),
            ]
            .padding(15)
            .spacing(5)
            .align_items(Alignment::Center)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
    fn update(&mut self, message: Message)
    {
        let path: &Path = Path::new("/proc/acpi/ibm/fan");
        let out;
        match message {
            Message::Off => out="Off",
            Message::Low => out="Low",
            Message::Medium => out="Medium",
            Message::High => out="High",
            Message::Max => out="Max",
            Message::Auto => out="Auto"
        }
        match echo(translate(out),path)
        {
            Ok(_) => self.err = None,
            Err(why) => self.err = Some(format!("Can't write to file: {:?}", why.kind()).to_string())
        }
        self.current = get_current(path);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message
{
    Off,
    Low,
    Medium,
    High,
    Max,
    Auto
}

fn translate(a: &str) -> &str
{
    let out;
    match a
    {
        "Off" => out = "level 0",
        "Low" => out = "level 2",
        "Medium" => out = "level 4",
        "High" => out = "level 7",
        "Max" => out = "level disengaged",
        "Auto" => out = "level auto",
        _ => out = "level auto"
    }
    return out;
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn get_current(path: &Path) -> String
{
    let s;
    match cat(path)
    {
        Ok(r) => s = r,
        Err(_) => return "".to_string()
    }
    let level: &str = s.split("\n").filter(|part| part.contains("level:")).last().unwrap().split("\t").into_iter().last().unwrap();
    return String::from(level);
}

fn main()
{
    State::run(
        Settings {
            window: iced::window::Settings {
                size: (375,150),
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }
    ).ok();
}

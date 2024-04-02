#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{keyboard::{key::Named, Key}, window, Application, Command, Settings, Size, Theme };
use update::{key_message::KeyMessage, Message};
use view::tab::Tab;

mod view;
mod update;
mod sudoku;
mod styles;

fn main() {
    let settings = Settings {
        window: window::Settings {
            min_size: Some(Size::new(1000., 900.)),
            size: Size::new(1000., 900.),
            ..window::Settings::default()
        },
        ..Settings::default()
    };
    
    SudokuSolver::run(settings).unwrap();
}


struct SudokuSolver {
    theme: Theme,
    // Counter for the tab internal ID
    tab_id_counter: u16,
    // Index of the active tab in the tabs vector
    active_tab: usize,
    tabs: Vec<Tab>,
}

impl iced::Application for SudokuSolver {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let app = Self {
            theme: Theme::Dark,
            tab_id_counter: 1,
            active_tab: 0,
            tabs: vec![Tab::new(0)],
        };

        (app, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        message.update(self)
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {       
        view::view(&self) 
    }

    fn title(&self) -> String {
        "Sudoku Solver".to_owned()
    }

    fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::keyboard::on_key_press(|key, _| {
            match key {
                Key::Character(char) => {
                    match char.parse::<u8>().ok()? {
                        num @ 0..=9 => {
                            Some(Message::Input(num))
                        }
                        _ => {None}
                    }
                }
                Key::Named(named_key) => {
                    match named_key {
                        Named::Tab => Some(KeyMessage::Tab.into()),
                        Named::ArrowUp => Some(KeyMessage::Up.into()),
                        Named::ArrowDown => Some(KeyMessage::Down.into()),
                        Named::ArrowLeft => Some(KeyMessage::Left.into()),
                        Named::ArrowRight => Some(KeyMessage::Right.into()),
                        Named::Escape => Some(Message::ClearSelected),
                        Named::Backspace => Some(Message::ClearInput),
                        _ => None
                    }
                }
                _ => None
            }
        })
    }
}
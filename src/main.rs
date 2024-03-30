
use iced::{keyboard::{key::Named, Key}, theme, widget::{self, button, column, container, row, text}, window, Application, Command, Length, Settings, Size, Theme };
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
    tab_id_counter: u16,
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
        // let mut column = column![].push(widget::Rule::horizontal(1)
        //     .style(theme::Rule::Custom(Box::new(ThickRule))))
        //     .align_items(iced::Alignment::Center);

        // for (rowindex,row) in self.sudoku.puzzle.iter().enumerate() {
        //     let mut row_widget = row![]
        //         .align_items(iced::Alignment::Center)
        //         .width(Length::Fill)
        //         .height(Length::Fill)
        //         .push(widget::Rule::vertical(1).style(theme::Rule::Custom(Box::new(ThickRule))));

        //     for (columnindex, &field) in row.iter().enumerate() {
        //         let text_size:u16 = 300 / self.sudoku.size as u16;
        //         let field_as_string = field.to_string();
        //         let number = if field == 0 {""} else {field_as_string.as_str()};
        //         let mut tile = button(text(number)
        //                 .height(Length::Fill)
        //                 .width(Length::Fill)
        //                 .size(text_size)
        //                 .horizontal_alignment(iced::alignment::Horizontal::Center)
        //                 .vertical_alignment(iced::alignment::Vertical::Center)
        //             )
        //             .width(Length::Fill)
        //             .height(Length::Fill)
        //             .style(theme::Button::Text)
        //             .on_press(Message::Select(rowindex, columnindex));
                
        //         if let Some((row, column)) = self.selected {
        //             if row == rowindex && column == columnindex {
        //                 tile = tile.on_press(Message::ClearSelected)
        //                     .style(theme::Button::Secondary)
        //             }
        //         }

        //         for (x, y) in &self.sudoku.conflicts {
        //             if rowindex == *x && columnindex == *y {
        //                 tile = tile.style(theme::Button::Destructive)
        //             }
        //         }

        //         if (columnindex + 1) % (self.sudoku.size as f32).sqrt() as usize == 0 {
        //             row_widget = row_widget.push(tile)
        //                 .push(widget::Rule::vertical(1).style(theme::Rule::Custom(Box::new(ThickRule))))
        //         } else {
        //             row_widget = row_widget.push(tile)
        //                 .push(widget::Rule::vertical(1))
        //         };
        //     }
        //     if (rowindex + 1) % (self.sudoku.size as f32).sqrt() as usize == 0 {
        //         column = column.push(row_widget)
        //             .push(widget::Rule::horizontal(1).style(theme::Rule::Custom(Box::new(ThickRule))))
        //     } else {
        //         column = column.push(row_widget).push(widget::Rule::horizontal(1))
        //     }
        // }

        // let mut number_buttons = row![]
        //     .height(Length::Shrink)
        //     .align_items(iced::Alignment::Center)
        //     .spacing(5)
        //     .push(button(text("C")
        //             .height(Length::Fill)
        //             .width(Length::Fill)
        //             .horizontal_alignment(iced::alignment::Horizontal::Center)
        //             .vertical_alignment(iced::alignment::Vertical::Center)
        //         )
        //         .height(50)
        //         .width(50)
        //         .on_press_maybe(self.selected
        //             .and_then(|_| Some(Message::Input(0)))    
        //         )
        //     );

        // for i in 1..=self.sudoku.size {
        //     let button = button(text(i)
        //             .height(Length::Fill)
        //             .width(Length::Fill)
        //             .horizontal_alignment(iced::alignment::Horizontal::Center)
        //             .vertical_alignment(iced::alignment::Vertical::Center)
        //         )
        //         .height(50)
        //         .width(50)
        //         .on_press_maybe(self.selected
        //             .and_then(|_| Some(Message::Input(i)))
        //         );
        //     number_buttons = number_buttons.push(button);
        // }

        // let solve_button = button(text("Solve")
        //         .horizontal_alignment(iced::alignment::Horizontal::Center)
        //         .vertical_alignment(iced::alignment::Vertical::Center)
        //     )
        //     .width(150)
        //     .height(60)
        //     .on_press(Message::Solve);
        // let clear_button = button(text("Clear")
        //         .horizontal_alignment(iced::alignment::Horizontal::Center)
        //         .vertical_alignment(iced::alignment::Vertical::Center)
        //     )
        //     .width(150)
        //     .height(60)
        //     .on_press(Message::Clear);

        // let control_buttons = row![clear_button, solve_button]
        //     .width(Length::Shrink)
        //     .spacing(20)
        //     .align_items(iced::Alignment::Center);

        // let buttons = column![number_buttons, control_buttons]
        //     .spacing(10)
        //     .align_items(iced::Alignment::Center);

        // let buttons_container = container(buttons)
        //     .center_x().height(Length::Shrink).width(Length::Fill);

        // let content = column![column, buttons_container]
        //     .spacing(20)
        //     .align_items(iced::Alignment::Center);
        // let content = container(content);
        // container(content).padding(20).into()
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
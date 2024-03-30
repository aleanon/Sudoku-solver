use iced::{theme, widget::{self, button, column, container, row, text}, Element, Length, Padding};

use crate::{styles::ThickRule, sudoku::Sudoku, update::{Message, SudokuSize}};





pub struct Tab {
    pub id: u16,
    pub sudoku: Option<Sudoku>,
    pub selected: Option<(usize, usize)>,
    pub solving: bool,
}

impl<'a> Tab {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            sudoku: None,
            selected: None,
            solving: false,
        }
    }

    pub fn view(&'a self) -> iced::Element<'a, Message> {
      let center:Element<'a, Message> = match &self.sudoku {
        Some(sudoku) => {
          let sudoku_grid = Self::sudoku_grid(self.solving, &self.selected, &sudoku);

          let mut number_buttons = row![]
              .height(Length::Shrink)
              .align_items(iced::Alignment::Center)
              .spacing(5)
              .push(button(text("C")
                      .height(Length::Fill)
                      .width(Length::Fill)
                      .horizontal_alignment(iced::alignment::Horizontal::Center)
                      .vertical_alignment(iced::alignment::Vertical::Center)
                  )
                  .height(50)
                  .width(50)
                  .on_press_maybe(self.selected
                      .and_then(|_| Some(Message::Input(0)))    
                  )
              );

          for i in 1..=sudoku.size {
              let button = button(text(i)
                      .height(Length::Fill)
                      .width(Length::Fill)
                      .horizontal_alignment(iced::alignment::Horizontal::Center)
                      .vertical_alignment(iced::alignment::Vertical::Center)
                  )
                  .height(50)
                  .width(50)
                  .on_press_maybe(self.selected
                      .and_then(|_| Some(Message::Input(i)))
                  );
              number_buttons = number_buttons.push(button);
          }

          let solve_button_text = if self.solving {"Solving"} else {"Solve"};
      
          let solve_button = button(text(solve_button_text)
                  .horizontal_alignment(iced::alignment::Horizontal::Center)
                  .vertical_alignment(iced::alignment::Vertical::Center)
              )
              .width(150)
              .height(60)
              .on_press_maybe(if self.solving {None} else {Some(Message::Solve)});

          let clear_button = button(text("Clear")
                  .horizontal_alignment(iced::alignment::Horizontal::Center)
                  .vertical_alignment(iced::alignment::Vertical::Center)
              )
              .width(150)
              .height(60)
              .on_press_maybe(if self.solving {None} else {Some(Message::Clear)});

          let control_buttons = row![clear_button, solve_button]
              .width(Length::Shrink)
              .spacing(20)
              .align_items(iced::Alignment::Center);

          let buttons = column![number_buttons, control_buttons]
            .align_items(iced::Alignment::Center)
            .spacing(10);

          let buttons_container = container(buttons)
              .center_x().height(Length::Shrink).width(Length::Fill);

          column![sudoku_grid, buttons_container].spacing(10).into()
        }
        None => {
          let nine_by_nine  = button(text("9 X 9")
            .vertical_alignment(iced::alignment::Vertical::Center)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .size(20)
          )
          .height(80)
          .width(250)
          .on_press(Message::NewSudoku(SudokuSize::NineByNine));

          let sixteen_by_sixteen = button(text("16 X 16")
            .vertical_alignment(iced::alignment::Vertical::Center)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .size(20)
          )
          .height(80)
          .width(250)
          .on_press(Message::NewSudoku(SudokuSize::SixteenBySixteen));

          let twentyfive_by_twentyfive = button(text("25 X 25")
            .vertical_alignment(iced::alignment::Vertical::Center)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .size(20)
          )
          .height(80)
          .width(250)
          .on_press(Message::NewSudoku(SudokuSize::TwentyfiveByTwentyfive));
            
          let buttons = column![nine_by_nine, sixteen_by_sixteen, twentyfive_by_twentyfive]
            .spacing(30)
            .width(Length::Shrink)
            .height(Length::Shrink);

          container(buttons)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(10)
            .into()
        }
      };

      container(center).padding(Padding {bottom: 10., left: 10., right: 10., top: 0.}).into()

    }

    fn sudoku_grid(solving: bool, selected: &Option<(usize, usize)>, sudoku: &'a Sudoku) -> Element<'a, Message> {
        let mut column = column![].push(widget::Rule::horizontal(1)
                .style(theme::Rule::Custom(Box::new(ThickRule))))
                .align_items(iced::Alignment::Center);

        for (rowindex,row) in sudoku.puzzle.iter().enumerate() {
            let mut row_widget = row![]
                .align_items(iced::Alignment::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .push(widget::Rule::vertical(1).style(theme::Rule::Custom(Box::new(ThickRule))));

            for (columnindex, &field) in row.iter().enumerate() {
                let text_size:u16 = 300 / sudoku.size as u16;
                let field_as_string = field.to_string();
                let number = if field == 0 {""} else {field_as_string.as_str()};
                let mut tile = button(text(number)
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .size(text_size)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(theme::Button::Text)
                    .on_press_maybe(if solving {None} else {Some(Message::Select(rowindex, columnindex))});
                
                if let Some((row, column)) = selected {
                    if *row == rowindex && *column == columnindex {
                        tile = tile.on_press(Message::ClearSelected)
                            .style(theme::Button::Secondary)
                    }
                }

                for (x, y) in &sudoku.conflicts {
                    if rowindex == *x && columnindex == *y {
                        tile = tile.style(theme::Button::Destructive)
                    }
                }

                if (columnindex + 1) % (sudoku.size as f32).sqrt() as usize == 0 {
                    row_widget = row_widget.push(tile)
                        .push(widget::Rule::vertical(1).style(theme::Rule::Custom(Box::new(ThickRule))))
                } else {
                    row_widget = row_widget.push(tile)
                        .push(widget::Rule::vertical(1))
                };
            }
            if (rowindex + 1) % (sudoku.size as f32).sqrt() as usize == 0 {
                column = column.push(row_widget)
                    .push(widget::Rule::horizontal(1).style(theme::Rule::Custom(Box::new(ThickRule))))
            } else {
                column = column.push(row_widget).push(widget::Rule::horizontal(1))
            }
        }
        column.into()
    }
}

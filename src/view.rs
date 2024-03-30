pub mod tab;

use iced::{theme, widget::{button, column, row, text}, Element, Length};

use crate::{styles, update::Message, SudokuSolver};

use self::tab::Tab;


pub fn view<'a>(appdata: &'a SudokuSolver) -> iced::Element<'a, Message> {
  let tabs = tab_row(&appdata.tabs, appdata.active_tab);

  let active_tab = appdata.tabs[appdata.active_tab].view();

  column![tabs, active_tab].spacing(10).into()
}



fn tab_row<'a>(tabs: &'a Vec<Tab>, active_tab: usize) -> Element<'a, Message> {
  let mut tab_row = row![].spacing(10).padding(10);

  for (i, ref tab) in tabs.iter().enumerate() {
    let tab_name = match tab.sudoku {
      Some(ref sudoku) => format!("{} X {}", sudoku.size, sudoku.size),
      None => format!("Tab {}", i+1),
    };

    let tab_name = text(tab_name)
      .horizontal_alignment(iced::alignment::Horizontal::Center)
      .vertical_alignment(iced::alignment::Vertical::Center)
      .height(Length::Fill)
      .width(Length::Fill)
      .size(12);

    let mut close_tab = button(text("x")
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .height(Length::Fill)
        .width(Length::Fill)
        .size(6)
      )
      .width(25)
      .height(25)
      .on_press(Message::CloseTab(i));

    if i == active_tab {
      close_tab = close_tab.style(theme::Button::custom(styles::ActiveTab))
    }

    let text_close_row = row![tab_name, close_tab]
      .spacing(5)
      .width(Length::Fill)
      .height(Length::Fill)
      .align_items(iced::Alignment::Center);

    let mut button = button(text_close_row)
      .width(80)
      .height(40)
      .on_press(Message::SetActiveTab(i));

    if i == active_tab {
      button = button.style(theme::Button::custom(styles::ActiveTab));
    }

    tab_row = tab_row.push(button)
  }   

  let new_tab_button = button(text("+")
    .horizontal_alignment(iced::alignment::Horizontal::Center)
    .vertical_alignment(iced::alignment::Vertical::Center)
    .height(Length::Fill)
    .width(Length::Fill)
    .size(16)
  )
  .width(40)
  .height(40)
  .on_press(Message::NewTab);

  tab_row.push(new_tab_button).into()  
}
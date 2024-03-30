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
  let mut tab_row = row![].spacing(10);

  for i in 0..tabs.len() {
    let tab_name = format!("Tab {}", i+1);
    let mut button = button(text(tab_name)
      .horizontal_alignment(iced::alignment::Horizontal::Center)
      .vertical_alignment(iced::alignment::Vertical::Center)
      .height(Length::Fill)
      .width(Length::Fill)
      .size(20)
    )
    .width(80)
    .height(40)
    .on_press(Message::SetActiveTab(i));

    if i == active_tab {
      button = button.style(theme::Button::custom(styles::ActiveTab))
    }

    tab_row = tab_row.push(button)
  }   

  let new_tab_button = button(text("+")
    .horizontal_alignment(iced::alignment::Horizontal::Center)
    .vertical_alignment(iced::alignment::Vertical::Center)
    .height(Length::Fill)
    .width(Length::Fill)
  )
  .width(40)
  .height(40)
  .on_press(Message::NewTab);

  tab_row.push(new_tab_button).into()  
}
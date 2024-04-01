pub mod tab;

use iced::{theme, widget::{button, column, container, row, text}, Element, Length};

use crate::{styles, update::Message, SudokuSolver};

use self::tab::Tab;


pub fn view<'a>(appdata: &'a SudokuSolver) -> iced::Element<'a, Message> {
  let tabs = tab_row(&appdata.tabs, appdata.active_tab);

  let active_tab = appdata.tabs[appdata.active_tab].view();

  column![tabs, active_tab].spacing(10).into()
}



fn tab_row<'a>(tabs: &'a Vec<Tab>, active_tab: usize) -> Element<'a, Message> {
  let mut tab_row = row![].spacing(10).align_items(iced::Alignment::Center).clip(true);

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
        .size(8)
      )
      .width(25)
      .height(25)
      .on_press(Message::CloseTab(i));

    if i == active_tab {
      close_tab = close_tab.style(theme::Button::custom(styles::ActiveTab))
    }


    let text_close_tab = row![tab_name, close_tab]
      .spacing(5)
      .width(Length::Fill)
      .height(Length::Fill)
      .align_items(iced::Alignment::Center);

    let mut button = button(text_close_tab)
      .width(Length::Fill)
      .height(Length::Fill)
      .on_press(Message::SetActiveTab(i));

    if i == active_tab {
      button = button.style(theme::Button::custom(styles::ActiveTab));
    }
    
    let button = container(button).height(40).width(80).max_width(80);


    tab_row = tab_row.push(button)
  }   

  let new_tab_button = button(text("+")
    .horizontal_alignment(iced::alignment::Horizontal::Center)
    .vertical_alignment(iced::alignment::Vertical::Center)
    .height(Length::Fill)
    .width(Length::Fill)
    .size(16)
  )
  .width(35)
  .height(35)
  .style(theme::Button::custom(styles::NewTabButton))
  .on_press(Message::NewTab);

  let space = iced::widget::Space::new(Length::Fill, 1);

  let theme_button = button(text("T")).height(40).width(40).on_press(Message::ToggleTheme);

  row![tab_row, new_tab_button, space, theme_button].align_items(iced::Alignment::Center).padding(10).spacing(10).into()
}
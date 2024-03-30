use iced::Command;

use crate::{sudoku, SudokuSolver};

use super::Message;


#[derive(Debug, Clone)]
pub enum KeyMessage {
  Tab,
  Up,
  Down,
  Right,
  Left,  
}


impl Into<Message> for KeyMessage {
  fn into(self) -> Message {
      Message::KeyMessage(self)
  }
}

impl<'a> KeyMessage {
  pub fn update(self, appdata: &'a mut SudokuSolver) -> Command<Message> {
    match self {
      Self::Tab => Self::tab(appdata),
      Self::Up => Self::up(appdata),
      Self::Down => Self::down(appdata),
      Self::Right => Self::right(appdata),
      Self::Left => Self::left(appdata),
    }
  }

  fn tab(appdata: &'a mut SudokuSolver) -> Command<Message> {
    let active_tab = &mut appdata.tabs[appdata.active_tab];

    if active_tab.solving {return Command::none()}

    let sudoku_maybe = &active_tab.sudoku;
    let selected = &mut active_tab.selected;

    if let Some(ref sudoku) = sudoku_maybe {
      if let Some((ref mut row, ref mut column)) = selected {
          let max_index = sudoku.size as usize -1;
          if *column < max_index {
              *column += 1
          } else if *row < max_index {
              *column = 0;
              *row += 1
          } else {*row = 0; *column = 0}
      } else {
          *selected = Some((0,0))
      }

    }

    Command::none()
  }

    fn up(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some((ref mut row, _)) = appdata.tabs[appdata.active_tab].selected {
          if *row > 0 {*row -= 1} 
      }

      Command::none()
    }

    fn down(appdata: &'a mut SudokuSolver) -> Command<Message> {
      let active_tab = &mut appdata.tabs[appdata.active_tab];
      let sudoku_maybe = &active_tab.sudoku;

      if let Some(sudoku) = sudoku_maybe {
        if let Some((ref mut row, _)) = active_tab.selected {
            if *row < (sudoku.size as usize) - 1 {*row += 1}
        }
      }

      Command::none()
    }

    fn left(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some((_, ref mut column)) = appdata.tabs[appdata.active_tab].selected {
          if *column > 0 {*column -= 1}
      }

      Command::none()
    }

    fn right(appdata: &'a mut SudokuSolver) -> Command<Message> {
      let active_tab = &mut appdata.tabs[appdata.active_tab];
      let sudoku_maybe = &active_tab.sudoku;

      if let Some(ref sudoku) = sudoku_maybe {
        if let Some((_, ref mut column)) = active_tab.selected {
            if *column < (sudoku.size as usize) - 1 {*column += 1}
        }
      }

      Command::none()
    }
}
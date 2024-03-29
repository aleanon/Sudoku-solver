use iced::Command;

use crate::{sudoku::Sudoku, SudokuSolver};



#[derive(Debug, Clone)]
pub enum Message {
    Tab,
    Up,
    Down,
    Right,
    Left,
    Solve,
    Clear,
    Input(u8),
    Select(usize, usize),
    ClearSelected,
    SolveResult(Option<Vec<Vec<u8>>>)
}

impl<'a> Message {
    pub fn update(self, appdata: &'a mut SudokuSolver) -> Command<Message> {
        match self {
            Message::Tab => Self::tab(appdata),
            Message::Up => Self::up(appdata),
            Message::Down => Self::down(appdata),
            Message::Left => Self::left(appdata),
            Message::Right => Self::right(appdata),
            Message::Solve => Self::solve(appdata),
            Message::Clear => Self::clear(appdata),
            Message::Input(input) => Self::input(input, appdata),
            Message::Select(row, column) => Self::select(row, column, appdata),
            Message::ClearSelected => Self::clearselected(appdata),
            Message::SolveResult(result) => Self::solve_result(result, appdata),
        }
    }
    
    fn tab(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some((ref mut row, ref mut column)) = appdata.selected {
          let max_index = appdata.sudoku.size as usize -1;
          if *column < max_index {
              *column += 1
          } else if *row < max_index {
              *column = 0;
              *row += 1
          } else {*row = 0; *column = 0}
      } else {
          appdata.selected = Some((0,0))
      }

      Command::none()
    }

    fn up(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some((ref mut row, _)) = appdata.selected {
          if *row > 0 {*row -= 1} 
      }

      Command::none()
    }

    fn down(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some((ref mut row, _)) = appdata.selected {
          if *row < (appdata.sudoku.size as usize) - 1 {*row += 1}
      }

      Command::none()
    }

    fn left(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some((_, ref mut column)) = appdata.selected {
          if *column > 0 {*column -= 1}
      }

      Command::none()
    }

    fn right(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some((_, ref mut column)) = appdata.selected {
          if *column < (appdata.sudoku.size as usize) - 1 {*column += 1}
      }

      Command::none()
    }

    fn solve(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some(conflicts) = appdata.sudoku.validate() {
          appdata.sudoku.conflicts = conflicts;
      } else {
          appdata.sudoku.conflicts.clear();
          let sudoku = appdata.sudoku.clone();
          return Command::perform(
              async move {sudoku.solve()}, 
              |result| Message::SolveResult(result)
          )
      }

      Command::none()
    }

    fn clear(appdata: &'a mut SudokuSolver) -> Command<Message> {
      for row in appdata.sudoku.puzzle.iter_mut() {
          for n in row {
              *n = 0
          }
      }
      appdata.sudoku.conflicts.clear();

      Command::none()
    }

    fn input(input: u8, appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some((row, column)) = appdata.selected {
          appdata.sudoku.puzzle[row][column] = input
      }
      if let Some(conflicts) = appdata.sudoku.validate() {
          appdata.sudoku.conflicts = conflicts
      } else {
          appdata.sudoku.conflicts.clear()
      }

      Command::none()
    }

    fn select(row: usize, column: usize, appdata: &'a mut SudokuSolver) -> Command<Message> {
      appdata.selected = Some((row, column));      

      Command::none()
    }

    fn clearselected(appdata: &'a mut SudokuSolver) -> Command<Message> {
      appdata.selected = None;

      Command::none()
    }

    fn solve_result(result: Option<Vec<Vec<u8>>>, appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some(puzzle) = result {
          appdata.sudoku = Sudoku::from_puzzle(puzzle)
      }

      Command::none()
    } 
}

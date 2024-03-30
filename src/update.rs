pub mod key_message;

use iced::{Command, Theme};

use crate::{styles::ActiveTab, sudoku::Sudoku, view::tab::Tab, SudokuSolver};

use self::key_message::KeyMessage;

#[derive(Debug, Clone)]
pub enum SudokuSize {
  NineByNine,
  SixteenBySixteen,
  TwentyfiveByTwentyfive,
}

#[derive(Debug, Clone)]
pub enum Message {
    KeyMessage(KeyMessage),
    ToggleTheme,
    NewSudoku(SudokuSize),
    SetActiveTab(usize),
    NewTab,
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
            Message::KeyMessage(key_message) => key_message.update(appdata),
            Message::Solve => Self::solve(appdata),
            Message::ToggleTheme => Self::toggle_theme(appdata),
            Message::NewSudoku(sudoku_size) => Self::new_sudoku(sudoku_size, appdata),
            Message::SetActiveTab(tab_id) => Self::set_active_tab(tab_id, appdata),
            Message::NewTab => Self::create_new_tab(appdata),
            Message::Clear => Self::clear(appdata),
            Message::Input(input) => Self::input(input, appdata),
            Message::Select(row, column) => Self::select(row, column, appdata),
            Message::ClearSelected => Self::clearselected(appdata),
            Message::SolveResult(result) => Self::solve_result(result, appdata),
        }
    }
    
    fn toggle_theme(appdata: &'a mut SudokuSolver) -> Command<Message> {
      match appdata.theme {
        Theme::Dark => appdata.theme = Theme::Light,
        Theme::Light => appdata.theme = Theme::Dark,
        _ => {}
      }

      Command::none()
    }

    fn new_sudoku(sudoku_size: SudokuSize, appdata: &'a mut SudokuSolver) -> Command<Message> {
      match sudoku_size {
        SudokuSize::NineByNine => appdata.tabs[appdata.active_tab].sudoku = Some(Sudoku::new(9)),
        SudokuSize::SixteenBySixteen => appdata.tabs[appdata.active_tab].sudoku = Some(Sudoku::new(16)),
        SudokuSize::TwentyfiveByTwentyfive => appdata.tabs[appdata.active_tab].sudoku = Some(Sudoku::new(25)),  
      }

      Command::none()
    }

    fn set_active_tab(tab_id: usize, appdata:&'a mut SudokuSolver) -> Command<Message> {
      appdata.active_tab = tab_id;

      Command::none()
    }

    fn create_new_tab(appdata: &'a mut SudokuSolver) -> Command<Message> {
      let tab = Tab::new();
      let tab_id = appdata.tabs.len();
      appdata.tabs.push(tab);
      appdata.active_tab = tab_id;

      Command::none()
    }

    
    fn solve(appdata: &'a mut SudokuSolver) -> Command<Message> {
      let active_tab = &mut appdata.tabs[appdata.active_tab];
      let sudoku_maybe = &mut active_tab.sudoku;
      if let Some(ref mut sudoku) = sudoku_maybe {
        if let Some(conflicts) = sudoku.validate() {
            sudoku.conflicts = conflicts;
        } else {
            active_tab.solving = true;
            active_tab.selected = None;
            sudoku.conflicts.clear();
            let sudoku = sudoku.clone();
            return Command::perform(
                async move {sudoku.solve()}, 
                |result| Message::SolveResult(result)
            )
        }

      }

      Command::none()
    }

    fn clear(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some(ref mut sudoku) = appdata.tabs[appdata.active_tab].sudoku {
        for row in sudoku.puzzle.iter_mut() {
            for n in row {
                *n = 0
            }
        }
        sudoku.conflicts.clear();
      }

      Command::none()
    }

    fn input(input: u8, appdata: &'a mut SudokuSolver) -> Command<Message> {
      let tab = &mut appdata.tabs[appdata.active_tab];
      let sudoku_maybe = &mut tab.sudoku;
      let selected  = &mut tab.selected;

      if let Some(ref mut sudoku) = sudoku_maybe {
        if let Some((row, column)) = selected {
            sudoku.puzzle[*row][*column] = input
        }
        if let Some(conflicts) = sudoku.validate() {
            sudoku.conflicts = conflicts
        } else {
            sudoku.conflicts.clear()
        }

      }

      Command::none()
    }

    fn select(row: usize, column: usize, appdata: &'a mut SudokuSolver) -> Command<Message> {
      let active_tab = &mut appdata.tabs[appdata.active_tab];
      if active_tab.solving {return Command::none()};

      active_tab.selected = Some((row, column));      

      Command::none()
    }

    fn clearselected(appdata: &'a mut SudokuSolver) -> Command<Message> {
      appdata.tabs[appdata.active_tab].selected = None;

      Command::none()
    }

    fn solve_result(result: Option<Vec<Vec<u8>>>, appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some(puzzle) = result {
          appdata.tabs[appdata.active_tab].sudoku = Some(Sudoku::from_puzzle(puzzle))
      }
      appdata.tabs[appdata.active_tab].solving = false;

      Command::none()
    } 
}

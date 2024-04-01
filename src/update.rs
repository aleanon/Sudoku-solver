pub mod key_message;


use iced::{Command, Theme};

use crate::{sudoku::Sudoku, view::tab::Tab, SudokuSolver};

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
    ClearInput,
    Select(usize, usize),
    ClearSelected,
    SolveResult(u16, Option<Vec<Vec<u8>>>),
    CloseTab(usize),
    AbortSolving,
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
            Message::ClearInput => Self::clear_input(appdata),
            Message::Select(row, column) => Self::select(row, column, appdata),
            Message::ClearSelected => Self::clearselected(appdata),
            Message::SolveResult(tab_id, result) => Self::solve_result(tab_id, result, appdata),
            Message::CloseTab(tab_id) => Self::close_tab(tab_id, appdata),
            Message::AbortSolving => Self::abort_solving(appdata),
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
      let tab = Tab::new(appdata.tab_id_counter);
      appdata.tab_id_counter += 1;
      appdata.tabs.push(tab);
      appdata.active_tab = appdata.tabs.len() -1;

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
            let tab_id = active_tab.id;
            sudoku.conflicts.clear();
            let sudoku = sudoku.clone();
            let (sender, receiver) = crossbeam::channel::bounded::<()>(1);
            active_tab.sender = Some(sender);

            return Command::perform(
                async move {
                  let thread = std::thread::spawn(
                    move || {
                      sudoku.solve(receiver.clone())
                    }
                  );

                  let result = thread.join().ok().and_then(|result| result);
                  (tab_id, result)
                }, 
                |(id, result)| Message::SolveResult(id, result)
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
          let current_input = sudoku.puzzle[*row][*column];
            if (current_input * 10) + input <= sudoku.size {
              sudoku.puzzle[*row][*column] = current_input * 10 + input
            } else {
              sudoku.puzzle[*row][*column] = input
            }
        }
        if let Some(conflicts) = sudoku.validate() {
            sudoku.conflicts = conflicts
        } else {
            sudoku.conflicts.clear()
        }

      }

      Command::none()
    }

    fn clear_input(appdata: &'a mut SudokuSolver) -> Command<Message> {
      let tab = &mut appdata.tabs[appdata.active_tab];
      let sudoku_maybe = &mut tab.sudoku;
      if let Some(ref mut sudoku) = sudoku_maybe {
        if let Some((row, column)) = tab.selected {
          sudoku.puzzle[row][column] = 0
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

    fn solve_result(tab_id: u16, mut result: Option<Vec<Vec<u8>>>, appdata: &'a mut SudokuSolver) -> Command<Message> {
      for tab in &mut appdata.tabs {
        if tab.id == tab_id {
          if let Some(puzzle) = result.take() {
              tab.sudoku = Some(Sudoku::from_puzzle(puzzle))
          }
          tab.solving = false;
          tab.sender = None;
        }
      }

      Command::none()
    }

    fn close_tab(tab_id: usize, appdata: &'a mut SudokuSolver) -> Command<Message> {
      if appdata.tabs.len() == 1 {
        appdata.tabs[0] = Tab::new(appdata.tab_id_counter);
        appdata.tab_id_counter += 1;
        return Command::none()
      }
      if appdata.tabs.len() -1 == appdata.active_tab {
        appdata.active_tab = appdata.active_tab -1
      }

      let tab = appdata.tabs.remove(tab_id);

      if let Some(ref sender) = tab.sender {
        sender.try_send(()).ok();
      }

      Command::none()
    }

    fn abort_solving(appdata: &'a mut SudokuSolver) -> Command<Message> {
      if let Some(ref sender) = appdata.tabs[appdata.active_tab].sender {
        sender.try_send(()).ok();
      }

      Command::none()
    } 
}

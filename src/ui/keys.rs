use crossterm::event::{self, KeyCode};

use crate::Error;
use crate::ui::{App, SortBy};

impl App {
    pub fn handle_key(&mut self) -> Result<bool, Error> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                KeyCode::Char('o') => {
                    if let Some(i) = self.state.selected() {
                        let _ = open::that(self.repo_url(i));
                    }
                }
                KeyCode::Char('y') => {
                    if let Some(i) = self.state.selected() {
                        let url = self.repo_url(i);
                        if let Some(clipboard) = &mut self.clipboard {
                            let _ = clipboard.set_text(url);
                        }
                    }
                }
                KeyCode::Char('e') => {
                    self.export_markdown();
                }
                KeyCode::Char('/') => {
                    self.filtering = true;
                    self.filter = Some(String::new());
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    if self.filtering {
                        self.filtering = false;
                        self.filter = None;
                        self.apply_filter();
                    } else {
                        return Ok(true); // signal quit
                    }
                }
                KeyCode::Char(c) if self.filtering => {
                    self.filter.get_or_insert_default().push(c);
                    self.apply_filter();
                }
                KeyCode::Backspace if self.filtering => {
                    if let Some(f) = &mut self.filter {
                        f.pop();
                        self.apply_filter();
                    }
                }
                KeyCode::Enter if self.filtering => {
                    self.filtering = false;
                }
                KeyCode::Char('1') => {
                    self.sort_by = SortBy::Name;
                    self.sort();
                }
                KeyCode::Char('2') => {
                    self.sort_by = SortBy::Stars;
                    self.sort();
                }
                KeyCode::Char('3') => {
                    self.sort_by = SortBy::Forks;
                    self.sort();
                }
                KeyCode::Char('4') => {
                    self.sort_by = SortBy::Created;
                    self.sort();
                }
                KeyCode::Char('5') => {
                    self.sort_by = SortBy::Updated;
                    self.sort();
                }
                KeyCode::Char('?') => {
                    self.show_help = !self.show_help;
                }
                _ => {}
            }
        }
        Ok(false)
    }
}

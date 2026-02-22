pub mod colors;
pub mod draw;
pub mod keys;

use chrono::{DateTime, Utc};
use ratatui::DefaultTerminal;
use ratatui::style::{self, Color};
use ratatui::widgets::{Paragraph, ScrollbarState, TableState};
use style::palette::tailwind;

use crate::Error;
use crate::stats::{Repo, Stats};
use crate::ui::colors::AppColor;

const ITEM_HEIGHT: usize = 1;

#[derive(Clone, Copy)]
enum SortBy {
    Name,
    Stars,
    Forks,
    Created,
    Updated,
}

pub struct App {
    state: TableState,
    items: Vec<Repo>,
    sort_by: SortBy,
    scroll_state: ScrollbarState,
    colors: AppColor,
    // filter
    filtered: Vec<usize>,   // indices into items
    filter: Option<String>, // None = no filter
    filtering: bool,        // true = user is typing
    // clipboard
    // Otherwise "clipboard was dropped very quickly"
    clipboard: Option<arboard::Clipboard>,
    // help
    show_help: bool,
}

impl App {
    pub fn new(stats: Stats) -> Self {
        let items = stats.repos;
        let filtered: Vec<usize> = (0..items.len()).collect();
        Self {
            state: TableState::default().with_selected(0),
            sort_by: SortBy::Name,
            scroll_state: ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
            colors: AppColor::new(),
            items,
            filtered,
            filter: None,
            filtering: false,
            clipboard: arboard::Clipboard::new().ok(),
            show_help: false,
        }
    }

    fn sort(&mut self) {
        match self.sort_by {
            SortBy::Name => self.items.sort_by(|a, b| a.name.cmp(&b.name)),
            SortBy::Stars => self.items.sort_by(|a, b| b.stars.cmp(&a.stars)),
            SortBy::Forks => self.items.sort_by(|a, b| b.forks.cmp(&a.forks)),
            SortBy::Created => self.items.sort_by(|a, b| a.created_at.cmp(&b.created_at)), // oldest first
            SortBy::Updated => self.items.sort_by(|a, b| b.pushed_at.cmp(&a.pushed_at)), // most recent first
        }
        self.state.select(Some(0));
        self.scroll_state = self.scroll_state.position(0);
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    fn apply_filter(&mut self) {
        let query = self.filter.as_deref().unwrap_or("").to_lowercase();
        self.filtered = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, r)| r.name.to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect();
        self.state.select(Some(0));
    }

    fn popularity_color(stars: u32) -> Color {
        if stars >= 10_000 {
            tailwind::LIME.c500 // very popular
        } else if stars >= 1_000 {
            tailwind::YELLOW.c500 // gaining traction
        } else {
            tailwind::WHITE // normal
        }
    }

    fn abandoned_color(pushed_at: DateTime<Utc>) -> Color {
        let days = (Utc::now() - pushed_at).num_days();
        if days >= (365 * 2) {
            tailwind::ORANGE.c600
        } else if days >= 365 {
            tailwind::YELLOW.c500
        } else {
            tailwind::WHITE
        }
    }

    fn repo_url(&self, i: usize) -> String {
        let item = &self.items[i];
        format!("https://github.com/{}/{}", item.owner, item.name)
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        terminal
            .draw(|f| f.render_widget(Paragraph::new("Fetching stats...").centered(), f.area()))?;

        loop {
            terminal.draw(|frame| self.render(frame))?;
            if self.handle_key()? {
                return Ok(());
            }
        }
    }
}

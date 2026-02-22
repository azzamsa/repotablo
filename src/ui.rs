use chrono::{DateTime, Utc};
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{self, Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{
    Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Table, TableState,
};
use ratatui::{DefaultTerminal, Frame};
use style::palette::tailwind;

use crate::Error;
use crate::stats::{RepoStats, ReposStats};

const INFO_TEXT: [&str; 2] = [
    "Sort by: (1) Name | (2) Stars | (3) Forks | (4) Age | (5) Updated",
    "(O) Open | (Y) Copy | (Esc) quit | (↑/j) move up | (↓/k) move down",
];

const ITEM_HEIGHT: usize = 1;

struct TableColors {
    row_fg: Color,
    selected_row_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new() -> Self {
        Self {
            // https://docs.rs/ratatui/latest/ratatui/prelude/style/palette/tailwind/index.html
            row_fg: tailwind::WHITE,
            selected_row_style_fg: tailwind::VIOLET.c900,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: tailwind::NEUTRAL.c600,
        }
    }
}

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
    items: Vec<RepoStats>,
    sort_by: SortBy,
    scroll_state: ScrollbarState,
    colors: TableColors,
    // filter
    filtered: Vec<usize>,   // indices into items
    filter: Option<String>, // None = no filter
    filtering: bool,        // true = user is typing
    // clipboard
    // Otherwise "clipboard was dropped very quickly"
    clipboard: Option<arboard::Clipboard>,
}

impl App {
    pub fn new(stats: ReposStats) -> Self {
        let items = stats.repos;
        let filtered: Vec<usize> = (0..items.len()).collect();
        Self {
            state: TableState::default().with_selected(0),
            sort_by: SortBy::Name,
            scroll_state: ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(),
            items,
            filtered,
            filter: None,
            filtering: false,
            clipboard: arboard::Clipboard::new().ok(),
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
                            return Ok(());
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
                    _ => {}
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = frame.area().layout_vec(&layout);

        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default();
        let selected_row_style = Style::default()
            .bg(self.colors.selected_row_style_fg)
            .fg(self.colors.row_fg);

        let header = ["Name", "Stars", "Forks", "License", "Age", "Updated"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.filtered.iter().enumerate().map(|(i, &idx)| {
            let data = &self.items[idx];
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            let cells = item.into_iter().enumerate().map(|(j, content)| {
                let cell = Cell::from(Text::from(content.to_string()));
                match j {
                    // stars
                    1 => cell.style(Style::new().fg(Self::popularity_color(data.stars))),
                    // fork
                    2 => cell.style(Style::new().fg(Self::popularity_color(data.stars))),
                    // Age
                    5 => cell.style(Style::new().fg(Self::abandoned_color(data.pushed_at))),
                    _ => cell,
                }
            });
            Row::new(cells)
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(1)
        });
        let table = Table::new(
            rows,
            [
                Constraint::Length(15), // Name
                Constraint::Length(8),  // Stars
                Constraint::Length(8),  // Forks
                Constraint::Length(15), // License
                Constraint::Length(15), // Age
                Constraint::Min(15),    // Updated
            ],
        )
        .header(header)
        .highlight_symbol("  ")
        .highlight_spacing(HighlightSpacing::Always)
        .row_highlight_style(selected_row_style);
        frame.render_stateful_widget(table, area, &mut self.state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .style(Style::new().fg(self.colors.footer_border_color))
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let text = if self.filtering {
            format!("Filter: {}_", self.filter.as_deref().unwrap_or(""))
        } else {
            INFO_TEXT.join("\n")
        };
        let info_footer = Paragraph::new(Text::from(text))
            .style(Style::new().fg(self.colors.row_fg))
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(self.colors.footer_border_color)),
            );
        frame.render_widget(info_footer, area);
    }
}

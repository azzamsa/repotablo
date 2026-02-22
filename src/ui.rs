use color_eyre::Result;
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

use crate::stats::{RepoStats, ReposStats};

const INFO_TEXT: &str = "Sort by: (1) name | (2) stars | (3) forks | (4) created | (5) updated";

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
}

impl App {
    pub fn new(stats: ReposStats) -> Self {
        let items = stats.repos;
        Self {
            state: TableState::default().with_selected(0),
            sort_by: SortBy::Name,
            scroll_state: ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(),
            items,
        }
    }

    fn sort(&mut self) {
        match self.sort_by {
            SortBy::Name => self.items.sort_by(|a, b| a.name.cmp(&b.name)),
            SortBy::Stars => self.items.sort_by(|a, b| b.stars.cmp(&a.stars)),
            SortBy::Forks => self.items.sort_by(|a, b| b.forks.cmp(&a.forks)),
            SortBy::Created => self.items.sort_by(|a, b| b.created_at.cmp(&a.created_at)), // oldest first
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

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                    KeyCode::Char('k') | KeyCode::Up => self.previous_row(),

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
        let layout = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]);
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
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(content.to_string())))
                .collect::<Row>()
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
        let info_footer = Paragraph::new(Text::from(INFO_TEXT))
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

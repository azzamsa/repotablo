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
use unicode_width::UnicodeWidthStr;

use crate::repo::Repo;

const INFO_TEXT: [&str; 2] = [
    "(1) by name  | (2) by stars",
    "(Esc) quit | (↑) move up | (↓) move down",
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
    Email,
}

pub struct App {
    state: TableState,
    items: Vec<Repo>,
    sort_by: SortBy,
    longest_item_lens: (u16, u16), // order is (name, email)
    scroll_state: ScrollbarState,
    colors: TableColors,
}

// impl Default for App {
//     fn default() -> Self {
//         Self::new(Vec::new())
//     }
// }

impl App {
    pub fn new(data: Vec<Repo>) -> Self {
        Self {
            state: TableState::default().with_selected(0),
            sort_by: SortBy::Name,
            longest_item_lens: constraint_len_calculator(&data),
            scroll_state: ScrollbarState::new((data.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(),
            items: data,
        }
    }

    fn sort(&mut self) {
        match self.sort_by {
            SortBy::Name => {
                self.items.sort_by(|a, b| a.name.cmp(&b.name));
            }
            SortBy::Email => {
                self.items.sort_by(|a, b| a.stars.cmp(&b.stars));
            }
        }

        // reset selection
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
                        self.sort_by = SortBy::Email;
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

        let header = ["Name", "Email"]
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
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 2),
                Constraint::Min(self.longest_item_lens.1),
            ],
        )
        .header(header)
        .highlight_symbol("  ")
        .highlight_spacing(HighlightSpacing::Always)
        .row_highlight_style(selected_row_style);
        // .block(Block::default().padding(ratatui::widgets::Padding::left(1)));

        frame.render_stateful_widget(t, area, &mut self.state);
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
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
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

fn constraint_len_calculator(items: &[Repo]) -> (u16, u16) {
    let name_len = items
        .iter()
        .map(Repo::name)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let stars_len = items
        .iter()
        .map(Repo::stars)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[expect(clippy::cast_possible_truncation)]
    (name_len as u16, stars_len as u16)
}

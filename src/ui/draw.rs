use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{self, Style};
use ratatui::text::Text;
use ratatui::widgets::{
    Block, BorderType, Cell, Clear, HighlightSpacing, Paragraph, Row, Scrollbar,
    ScrollbarOrientation, Table,
};
use ratatui::{DefaultTerminal, Frame};
use style::palette::tailwind;

use crate::Error;
use crate::ui::App;

const INFO_TEXT: [&str; 2] = [
    "Sort by: (1) Name | (2) Stars | (3) Forks | (4) Age | (5) Updated",
    "(O) Open | (Y) Copy | (?) Help",
];

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = frame.area().layout_vec(&layout);

        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);

        if self.show_help {
            self.render_help(frame);
        }
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

    fn render_help(&self, frame: &mut Frame) {
        let area = frame.area();

        // centered popup
        let popup = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };
        let text = [
            "  Keybindings",
            "  ──────────────────────────",
            "  1-5    Sort by column",
            "  /      Filter repos",
            "  o      Open in browser",
            "  y      Yank URL to clipboard",
            "  j/↓    Move down",
            "  k/↑    Move up",
            "  ?      Toggle this help",
            "  q/Esc  Quit",
        ]
        .join("\n");

        let block = Paragraph::new(text).block(
            Block::bordered()
                .title(" Help ")
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(tailwind::VIOLET.c400)),
        );

        frame.render_widget(Clear, popup); // clears background behind popup
        frame.render_widget(block, popup);
    }
}

pub fn draw_loading(
    terminal: &mut DefaultTerminal,
    current: usize,
    total: usize,
) -> Result<(), Error> {
    terminal.draw(|f| {
        let area = f.area();
        let vertical = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(area);
        f.render_widget(
            Paragraph::new(format!("Fetching {}/{}...", current, total)).centered(),
            vertical[1],
        )
    })?;
    Ok(())
}

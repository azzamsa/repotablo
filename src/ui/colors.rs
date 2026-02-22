use ratatui::style::{Color, palette::tailwind};

pub struct AppColor {
    pub row_fg: Color,
    pub selected_row_style_fg: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
    pub footer_border_color: Color,
}

impl Default for AppColor {
    fn default() -> Self {
        Self::new()
    }
}

impl AppColor {
    pub const fn new() -> Self {
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

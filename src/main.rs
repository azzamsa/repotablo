use color_eyre::Result;

use ratatui::widgets::Paragraph;
use repotablo::{stats::ReposStats, ui::App};

async fn run() -> Result<()> {
    let mut terminal = ratatui::init();

    terminal.draw(|f| f.render_widget(Paragraph::new("Fetching stats...").centered(), f.area()))?;

    let repos = vec![("DioxusLabs", "dioxus"), ("emilk", "egui")];
    let stats = ReposStats::fetch(repos).await?;

    App::new(stats).run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    run().await?;

    Ok(())
}

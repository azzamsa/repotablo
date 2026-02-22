use color_eyre::Result;

use ratatui::widgets::Paragraph;
use repotablo::{input::open_editor, stats::ReposStats, ui::App};

async fn run() -> Result<()> {
    let repos = open_editor()?;
    let repos: Vec<(&str, &str)> = repos
        .iter()
        .map(|(o, r)| (o.as_str(), r.as_str()))
        .collect();

    // Init ratatui AFTER editor closes, otherwise they fight for terminal control.
    let mut terminal = ratatui::init();

    let result = async {
        terminal
            .draw(|f| f.render_widget(Paragraph::new("Fetching stats...").centered(), f.area()))?;
        let stats = ReposStats::fetch(repos).await?;
        App::new(stats).run(&mut terminal)?;
        Ok(())
    }
    .await;

    ratatui::restore();
    result
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    run().await?;

    Ok(())
}

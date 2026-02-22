use color_eyre::Result;

use repotablo::{stats::ReposStats, ui::App};

async fn run() -> Result<()> {
    let repos = vec![("DioxusLabs", "dioxus"), ("emilk", "egui")];
    let stats = ReposStats::fetch(repos).await?;

    ratatui::run(|terminal| App::new(stats).run(terminal))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    run().await?;

    Ok(())
}

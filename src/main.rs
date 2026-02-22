use clap::Parser;
use octocrab::Octocrab;
use ratatui::widgets::Paragraph;

use repotablo::{Error, cli::Opts, input::get_repos, stats::ReposStats, ui::App};

async fn run() -> Result<(), Error> {
    let opts = Opts::parse();

    let oct = if let Some(token) = opts.github_token {
        Octocrab::builder().personal_token(token).build()?
    } else {
        Octocrab::default()
    };

    let repos = get_repos(opts.input).await?;
    let repos: Vec<(&str, &str)> = repos
        .iter()
        .map(|(o, r)| (o.as_str(), r.as_str()))
        .collect();

    // Init ratatui after editor closes, otherwise they fight for terminal control.
    let mut terminal = ratatui::init();

    let result = async {
        terminal
            .draw(|f| f.render_widget(Paragraph::new("Fetching stats...").centered(), f.area()))?;
        let stats = ReposStats::fetch(oct, repos).await?;
        App::new(stats).run(&mut terminal)?;
        Ok(())
    }
    .await;

    ratatui::restore();
    result
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    run().await?;
    Ok(())
}

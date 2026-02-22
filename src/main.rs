use clap::Parser;
use octocrab::Octocrab;

use repotablo::{
    Error,
    cli::Opts,
    input::get_repos,
    stats::Stats,
    ui::{App, draw::draw_loading},
};

async fn run() -> Result<(), Error> {
    let opts = Opts::parse();
    let oct = if let Some(token) = opts.github_token {
        Octocrab::builder().personal_token(token).build()?
    } else {
        Octocrab::default()
    };

    let repos = get_repos(opts.input).await?;

    // Init ratatui after editor closes, otherwise they fight for terminal control.
    let mut terminal = ratatui::init();

    let result = async {
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);

        let fetch_task = tokio::spawn({
            let repos = repos.clone();
            async move { Stats::fetch(&oct, repos, tx).await }
        });

        while let Some((current, total)) = rx.recv().await {
            draw_loading(&mut terminal, current, total)?;
        }

        let stats = fetch_task
            .await
            .map_err(|e| Error::Internal(e.to_string()))??;
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

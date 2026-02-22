use color_eyre::Result;

use repotablo::{repo::generate_fake_names, ui::App};

fn main() -> Result<()> {
    color_eyre::install()?;
    let repos = generate_fake_names();
    ratatui::run(|terminal| App::new(repos).run(terminal))
}

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Repotablo",
    version,
    about = "Repotablo [Rank repositories, fast!]",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/repotablo/issues"
)]
#[derive(Debug)]
pub struct Opts {
    /// URL or local file path (opens $EDITOR if not provided)
    pub input: Option<String>,

    /// GitHub token to avoid rate limiting
    #[clap(long, env)]
    pub github_token: Option<String>,
}

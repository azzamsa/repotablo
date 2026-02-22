use miette::Diagnostic;
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error("GitHub API rate limit exceeded")]
    #[diagnostic(
        code(repotablo::rate_limit),
        help("Set GITHUB_TOKEN env var: `export GITHUB_TOKEN=your_token`"),
        url(
            "https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token"
        )
    )]
    RateLimit,

    #[error("GitHub error: {0}")]
    GitHub(#[from] octocrab::Error),
}

impl std::convert::From<std::env::VarError> for Error {
    fn from(_err: std::env::VarError) -> Self {
        Self::NotFound("env var not found".into())
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

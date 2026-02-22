use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InvalidArgument(String),
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

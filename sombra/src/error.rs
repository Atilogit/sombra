use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Error while deserializing: {0}")]
    Deserializer(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(StatusCode),
    #[error("HTML parsing error: {0}")]
    Html(#[from] tl::ParseError),
    #[error("Profile parsing error")]
    Parse,
}

impl Error {
    pub fn result_from_status(code: StatusCode, expected: Option<StatusCode>) -> Result<()> {
        let expected = match expected {
            Some(code) => code,
            None => StatusCode::OK,
        };

        if expected == code {
            Ok(())
        } else {
            Err(Self::Http(code))
        }
    }

    #[must_use]
    pub fn parse() -> Self {
        eprintln!("{}", std::backtrace::Backtrace::capture());
        Self::Parse
    }
}

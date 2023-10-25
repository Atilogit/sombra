use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Sombra(#[from] sombra::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Sombra(e) => match e {
                sombra::Error::Http(status) => status.into_response(),
                sombra::Error::Request(_)
                | sombra::Error::Deserializer(_)
                | sombra::Error::Html(_)
                | sombra::Error::Parse => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
        }
    }
}

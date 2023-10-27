use poem::http::StatusCode;
use poem_openapi::ApiResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ApiResponse)]
pub enum Error {
    #[oai(status = 500)]
    Internal,
    #[oai(status = 404)]
    NotFound,
}

impl From<sombra::Error> for Error {
    fn from(e: sombra::Error) -> Self {
        match e {
            sombra::Error::Http(StatusCode::NOT_FOUND) => Self::NotFound,
            sombra::Error::Http(_)
            | sombra::Error::Request(_)
            | sombra::Error::Deserializer(_)
            | sombra::Error::Html(_)
            | sombra::Error::Battletag(_)
            | sombra::Error::Parse => {
                tracing::error!(error = ?e, "internal error");
                Self::Internal
            }
        }
    }
}

use axum::http::StatusCode;

pub type Result<T,E = Error> = core::result::Result<T,E>;

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum Error {
    #[error("internal server error")]
    InternalServer,
    #[error("not found")]
    NotFound,
    #[error("bad request:")]
    BadRequest(String)
}

impl Error {
    pub fn status_code(&self) -> StatusCode{
        match self {
            Self::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}
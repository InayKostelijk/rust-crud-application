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

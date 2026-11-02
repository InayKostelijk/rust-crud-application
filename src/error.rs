use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::error::{DatabaseError};

pub type Result<T,E = Error> = core::result::Result<T,E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal server error")]
    InternalServer,
    #[error("not found")]
    NotFound,
    #[error("bad request:")]
    BadRequest,
    #[error("database error")]
    //https://hda.daz.is/forms-and-errors/error-handling/ kijken of dit de manier is
    Database(#[from] sqlx::Error)


}


    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            let status = match &self {
                Self::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
                Self::NotFound => StatusCode::NOT_FOUND,
                Self::BadRequest => StatusCode::BAD_REQUEST,
                Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, self.to_string()).into_response()
        }
}
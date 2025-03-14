use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::BcryptError;
use serde_json::json;
use tokio::task::JoinError;
use wither::WitherError;

#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest{}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}


#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub enum Error {
    #[error("{0}")]
    Wither(#[from] WitherError),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    RunSyncTask(#[from] JoinError),

    #[error("{0}")]
    HashPassword(#[from] BcryptError),
}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        match *self {
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 40003),
            Error::Wither(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5002),
            Error::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5005),
            Error::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5006),
        }
    }
    pub fn bad_request() -> Self {
        Error::BadRequest(BadRequest {})
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let body = Json(json!({
            "code": code,
            "message": message
        }));
        (status_code, body).into_response()
    }
}

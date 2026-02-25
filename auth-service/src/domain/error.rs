use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use color_eyre::Report;
use serde::{Deserialize, Serialize};
use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthAPIError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Incorrect credentials")]
    IncorrectCredentials,
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        log_error_chain(&self);

        let status = match &self {
            AuthAPIError::UserAlreadyExists => StatusCode::CONFLICT,
            AuthAPIError::InvalidCredentials => StatusCode::BAD_REQUEST,
            AuthAPIError::MissingToken => StatusCode::BAD_REQUEST,
            AuthAPIError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthAPIError::UnexpectedError(e) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthAPIError::IncorrectCredentials => StatusCode::UNAUTHORIZED,
        };

        let body = Json(ErrorResponse { error: self.to_string() });

        (status, body).into_response()
    }
}

fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator = "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}

use crate::app_state::AppState;
use crate::domain::data_stores::UserStoreError;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let user_store = state.user_store.read().await;

    user_store
        .validate_user(&email, &password)
        .await
        .map_err(|err| match err {
            UserStoreError::InvalidCredentials => AuthAPIError::IncorrectCredentials,
            _ => AuthAPIError::InvalidCredentials,
        })?;

    Ok(StatusCode::OK.into_response())
}

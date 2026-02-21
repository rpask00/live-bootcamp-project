use crate::app_state::AppState;
use crate::domain::data_stores::UserStoreError;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::hashed_password::HashedPassword;
use crate::domain::user::User;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = HashedPassword::parse(request.password)
        .await
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let mut user_store = state.user_store.write().await;

    if user_store.get_user(&email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }
    let user = User::new(email, password, request.requires_2fa);

    user_store.add_user(user).await.map_err(|err| match err {
        UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
        UserStoreError::InvalidCredentials => AuthAPIError::InvalidCredentials,
        _ => AuthAPIError::UnexpectedError,
    })?;
    let response = Json(SignupResponse {
        message: "User signed up successfully".into(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    password: String,
    email: String,
    #[serde(rename = "requires2FA")]
    requires_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

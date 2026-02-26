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
use color_eyre::eyre::eyre;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

#[tracing::instrument(name = "Signup", skip_all)]
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email.expose_secret().into()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = HashedPassword::parse(request.password.into())
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
        e => AuthAPIError::UnexpectedError(eyre!(e)),
    })?;
    let response = Json(SignupResponse {
        message: "User signed up successfully".into(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    password: SecretString,
    email: SecretString,
    #[serde(rename = "requires2FA")]
    requires_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

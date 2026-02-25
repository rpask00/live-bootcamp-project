use crate::app_state::AppState;
use crate::domain::error::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::utils::constants::env::JWT_COOKIE_NAME;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::cookie::Cookie;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct VerifyTokenRequest {
    token: String,
}

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    if validate_token(&request.token).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let jwt = Cookie::new(JWT_COOKIE_NAME, request.token);

    let is_token_banned = state
        .banned_token_store
        .read()
        .await
        .contains_token(jwt.value().as_ref())
        .await
        .map_err(|e| AuthAPIError::UnexpectedError(eyre!(e)))?;

    if is_token_banned {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    Ok(StatusCode::OK.into_response())
}

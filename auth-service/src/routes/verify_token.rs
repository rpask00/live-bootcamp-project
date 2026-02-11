use crate::app_state::AppState;
use crate::domain::error::AuthAPIError;
use crate::utils::auth::validate_token;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
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

    Ok(StatusCode::OK.into_response())
}

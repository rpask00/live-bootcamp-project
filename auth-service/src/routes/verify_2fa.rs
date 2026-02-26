use crate::app_state::AppState;
use crate::domain::data_stores::{LoginAttemptId, TwoFACode};
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::routes::{handle_no_2fa, LoginResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::CookieJar;
use color_eyre::eyre::eyre;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String,
}

#[tracing::instrument(name = "Verify 2FA Code", skip_all)]
pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
        Ok(login_attempt_id) => login_attempt_id,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
        Ok(two_fa_code) => two_fa_code,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let (_login_attempt_id, _two_fa_code) = match state.two_fa_code_store.read().await.get_code(&email).await {
        Ok(result) => result,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    if _login_attempt_id != login_attempt_id {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    if _two_fa_code != two_fa_code {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    if let Err(e) = state.two_fa_code_store.write().await.remove_code(&email).await {
        return (jar, Err(AuthAPIError::UnexpectedError(eyre!(e))));
    }

    handle_no_2fa(&email, jar).await
}

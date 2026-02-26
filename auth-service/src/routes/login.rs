use crate::app_state::AppState;
use crate::domain::data_stores::{LoginAttemptId, TwoFACode};
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::utils::auth::generate_auth_cookie;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use color_eyre::eyre::eyre;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: SecretString,
    password: SecretString,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

#[tracing::instrument(name = "Login", skip_all)]
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email.expose_secret().into()) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = state.user_store.read().await;
    if user_store
        .validate_user(&email, &request.password.expose_secret())
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(e) => return (jar, Err(AuthAPIError::UnexpectedError(eyre!(e)))),
    };

    let jar = jar.add(auth_cookie);

    // Handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

#[tracing::instrument(name = "Handle 2FA flow", skip_all)]
async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    if let Err(e) = state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(eyre!(e))));
    }

    if let Err(e) = state
        .email_client
        .read()
        .await
        .send_email(
            email,
            "Login attempt",
            format!("Your 2FA code: {}", two_fa_code.as_ref()).as_str(),
        )
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(eyre!(e))));
    }

    (
        jar,
        Ok((
            StatusCode::PARTIAL_CONTENT,
            Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
                message: "2FA required".to_string(),
                login_attempt_id: login_attempt_id.0.expose_secret().to_string(),
            })),
        )),
    )
}

#[tracing::instrument(name = "Handle no 2FA flow", skip_all)]
pub async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    let cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(e) => return (jar, Err(AuthAPIError::UnexpectedError(eyre!(e)))),
    };

    let updated_jar = jar.add(cookie);

    (updated_jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))
}

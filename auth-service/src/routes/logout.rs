use crate::domain::error::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::utils::constants::env::JWT_COOKIE_NAME;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::{cookie, CookieJar};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let jwt = match jar.get("jwt") {
        None => return (jar, Err(AuthAPIError::InvalidCredentials)),
        Some(jwt) => jwt,
    };

    if validate_token(&jwt.value()).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let jar = jar.remove(cookie::Cookie::from(JWT_COOKIE_NAME));

    (jar, Ok(StatusCode::OK.into_response()))
}

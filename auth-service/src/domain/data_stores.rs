use crate::domain::email::Email;
use crate::domain::user::User;
use axum_extra::extract::cookie::Cookie;
use rand::{rng, Rng};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<&User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &str) -> Result<(), UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn ban_token(&mut self, token: &Cookie) -> ();
    async fn is_token_banned(&self, token: &Cookie) -> bool;
}

// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        // Use the `parse_str` function from the `uuid` crate to ensure `id` is a valid UUID
        match Uuid::parse_str(id.as_str()) {
            Ok(_uuid) => Ok(LoginAttemptId(_uuid.to_string())),
            Err(_) => Err("Failed to parse id".to_string()),
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        // Use the `uuid` crate to generate a random version 4 UUID
        LoginAttemptId(Uuid::new_v4().to_string())
    }
}

// TODO: Implement AsRef<str> for LoginAttemptId
impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        // Ensure `code` is a valid 6-digit code
        match code.len() {
            6 => Ok(TwoFACode(code)),
            _ => Err("Code is invalid".to_string()),
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        // Use the `rand` crate to generate a random 2FA code.
        // The code should be 6 digits (ex: 834629)
        TwoFACode(rng().random_range(100_000..=999_999).to_string())
    }
}

// TODO: Implement AsRef<str> for TwoFACode

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

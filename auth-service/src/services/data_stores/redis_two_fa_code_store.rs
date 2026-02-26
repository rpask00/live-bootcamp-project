use color_eyre::eyre::Context;
use redis::{Commands, Connection};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(RwLock::new(conn)),
        }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    #[tracing::instrument(name = "Add code into Redis 2FA Store", skip_all)]
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        // 2. Create a TwoFATuple instance.
        let two_fa_tuple = TwoFATuple(code.as_ref().to_owned(), login_attempt_id.0.expose_secret().to_owned());
        // 3. Use serde_json::to_string to serialize the TwoFATuple instance into a JSON string.
        let two_fa_tuple_str = serde_json::to_string(&two_fa_tuple)
            .wrap_err("Failed to serialize 2FA tuple")
            .map_err(TwoFACodeStoreError::UnexpectedError)?;
        // Return TwoFACodeStoreError::UnexpectedError if serialization fails.
        // 4. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).

        // The value should be the serialized 2FA tuple.
        // The expiration time should be set to TEN_MINUTES_IN_SECONDS.
        // Return TwoFACodeStoreError::UnexpectedError if casting fails or the call to set_ex fails.
        self.conn
            .write()
            .await
            .set_ex(get_key(&email), two_fa_tuple_str, TEN_MINUTES_IN_SECONDS)
            .wrap_err("Failed to set 2FA code in Redis") // New!
            .map_err(TwoFACodeStoreError::UnexpectedError)
    }

    #[tracing::instrument(name = "Remove code from Redis 2FA Store", skip_all)]
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        // 2. Call the del command on the Redis connection to delete the 2FA code entry.
        // Return TwoFACodeStoreError::UnexpectedError if the operation fails.
        self.conn
            .write()
            .await
            .del(get_key(&email))
            .wrap_err("Failed to delete 2FA code from Redis") // New!
            .map_err(TwoFACodeStoreError::UnexpectedError)
    }

    #[tracing::instrument(name = "Get code from Redis 2FA Store", skip_all)]
    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        // 2. Call the get command on the Redis connection to get the value stored for the key.
        // Return TwoFACodeStoreError::LoginAttemptIdNotFound if the operation fails.
        let result: String = self
            .conn
            .write()
            .await
            .get(get_key(&email))
            .map_err(|_| TwoFACodeStoreError::LoginAttemptIdNotFound)?;
        // If the operation succeeds, call serde_json::from_str to parse the JSON string into a TwoFATuple.
        let two_fa_tuple = serde_json::from_str::<TwoFATuple>(&result)
            .wrap_err("Failed to deserialize 2FA tuple") // New!
            .map_err(TwoFACodeStoreError::UnexpectedError)?;

        let two_fa_code = TwoFACode::parse(two_fa_tuple.0).map_err(TwoFACodeStoreError::UnexpectedError)?;
        let login_attempt_id = LoginAttemptId::parse(two_fa_tuple.1).map_err(TwoFACodeStoreError::UnexpectedError)?;

        // Then, parse the login attempt ID string and 2FA code string into a LoginAttemptId and TwoFACode type respectively.
        // Return TwoFACodeStoreError::UnexpectedError if parsing fails.

        Ok((login_attempt_id, two_fa_code))
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.0.expose_secret())
}

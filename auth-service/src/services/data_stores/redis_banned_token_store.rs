use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};
use color_eyre::eyre::Context;
use redis::{Commands, Connection};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(RwLock::new(conn)),
        }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    #[tracing::instrument(name = "Add banned token into Redis Store", skip_all)]
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        let key = get_key(&token);
        // 2. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).
        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .wrap_err("Failed to cast i64 into u64")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        // The value should simply be a `true` (boolean value).
        let _: () = self
            .conn
            .write()
            .await
            .set_ex(key, true, ttl)
            .wrap_err("Failed to set key in Store")
            .map_err(BannedTokenStoreError::UnexpectedError)?;
        // The expiration time should be set to TOKEN_TTL_SECONDS.
        // NOTE: The TTL is expected to be a u64 so you will have to cast TOKEN_TTL_SECONDS to a u64.
        // Return BannedTokenStoreError::UnexpectedError if casting fails or the call to set_ex fails.

        Ok(())
    }

    #[tracing::instrument(name = "Check if token is banned", skip_all)]
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        // Check if the token exists by calling the exists method on the Redis connection
        let key = get_key(&token);

        self.conn
            .write()
            .await
            .exists::<String, bool>(key)
            .wrap_err("Failed read from the Store to check if value for key exists")
            .map_err(BannedTokenStoreError::UnexpectedError)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}

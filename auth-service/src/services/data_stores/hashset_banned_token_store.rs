use crate::domain::data_stores::{BannedTokenStore, BannedTokenStoreError};
use secrecy::{ExposeSecret, SecretString};
use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: SecretString) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token.expose_secret().to_owned());

        Ok(())
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token))
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::email::Email;
    use crate::utils::auth::generate_auth_cookie;

    #[tokio::test]
    async fn test_ban_token() {
        let mut store = HashsetBannedTokenStore::default();
        let jwt = generate_auth_cookie(&Email::parse("test@test.pl".into()).unwrap()).unwrap();
        store.add_token(jwt.value().into()).await.expect("Failed to s add token.");

        let is_banned = store.contains_token(jwt.value().as_ref()).await.unwrap();

        assert!(is_banned);
    }
}

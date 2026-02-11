use crate::domain::data_stores::BannedTokenStore;
use axum_extra::extract::cookie::Cookie;
use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn ban_token(&mut self, token: &Cookie) -> () {
        self.banned_tokens.insert(token.value().to_string());
    }

    async fn is_token_banned(&self, token: &Cookie) -> bool {
        self.banned_tokens.contains(token.value())
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
        let token = generate_auth_cookie(&Email::parse("test@test.pl".to_string()).unwrap()).unwrap();
        store.ban_token(&token).await;

        let is_banned = store.is_token_banned(&token).await;

        assert!(is_banned);
    }
}

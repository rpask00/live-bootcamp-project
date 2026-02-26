use crate::domain::data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::domain::email::Email;
use color_eyre::eyre::eyre;
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    two_fa_store: HashMap<Email, (TwoFACode, LoginAttemptId)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.two_fa_store.insert(email, (code, login_attempt_id));

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        if self.two_fa_store.contains_key(email) {
            self.two_fa_store.remove(email);
        } else {
            return Err(TwoFACodeStoreError::UnexpectedError(eyre!("Code not found for given email!")));
        }

        Ok(())
    }

    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.two_fa_store.get(email) {
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
            Some((code, login_attempt_id)) => Ok((login_attempt_id.clone(), code.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
    use crate::domain::email::Email;
    use crate::services::data_stores::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[tokio::test]
    pub async fn test_add_code() {
        let mut store = HashmapTwoFACodeStore::default();

        let random_email = Email::parse(SafeEmail().fake()).unwrap();
        let code = TwoFACode::parse("123123".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();

        let result = store.add_code(random_email, login_attempt_id, code).await;
        assert!(result.is_ok());

        let failed_code = TwoFACode::parse("123".to_string());
        assert!(failed_code.is_err());
    }

    #[tokio::test]
    pub async fn test_get_code() {
        let mut store = HashmapTwoFACodeStore::default();

        let random_email = Email::parse(SafeEmail().fake()).unwrap();
        let code = TwoFACode::parse("123123".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();

        store
            .add_code(random_email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .expect("Adding code failed for some reason!");

        let result = store.get_code(&random_email).await;

        assert!(result.is_ok());

        let (_login_attempt_id, _code) = result.expect("Failed to receive code and login_attempt_id");

        assert_eq!(code, _code);
        assert_eq!(login_attempt_id, _login_attempt_id);
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::parse("123456".to_string()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .unwrap();

        // Remove
        let result = store.remove_code(&email).await;
        assert!(result.is_ok());

        // Verify it's gone
        let result = store.get_code(&email).await;
        assert!(matches!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound)));
    }
}

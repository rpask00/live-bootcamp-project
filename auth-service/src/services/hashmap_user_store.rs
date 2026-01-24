use crate::domain::user::User;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.

        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        };

        self.users.insert(user.email.clone(), user);

        Ok(())
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.users.get(email).ok_or(UserStoreError::UserNotFound)?;

        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        };

        Ok(())
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();

        let user = User::new("test@test.pl".into(), "test".into(), false);
        let result = store.add_user(user.clone());
        assert_eq!(result.unwrap(), (), "User should be added successfully");

        let result = store.add_user(user.clone());

        assert_eq!(
            result.expect_err("Result should be error"),
            UserStoreError::UserAlreadyExists,
            "Error UserAlreadyExists should be raised"
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@test.pl".into(), "test".into(), false);
        store.add_user(user.clone());

        let _user = store.get_user(&user.email);

        assert_eq!(
            _user.is_err(),
            false,
            "User should be returned successfully"
        );

        let _user = _user.unwrap();

        assert_eq!(_user, &user, "Correct user should be returned");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@test.pl".into(), "test".into(), false);
        store.add_user(user.clone()).unwrap();

        // Successful validation
        let res = store.validate_user(&user.email, "test");
        assert_eq!(res.unwrap(), (), "Valid credentials should return Ok(())");

        // Invalid password
        let res = store.validate_user(&user.email, "wrong_password");
        assert_eq!(
            res.expect_err("Result should be error"),
            UserStoreError::InvalidCredentials,
            "Incorrect password should return InvalidCredentials"
        );

        // Non-existent user
        let res = store.validate_user("noone@example.com", "whatever");
        assert_eq!(
            res.expect_err("Result should be error"),
            UserStoreError::UserNotFound,
            "Unknown email should return UserNotFound"
        );
    }
}

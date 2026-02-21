use crate::domain::email::Email;
use crate::domain::hashed_password::HashedPassword;

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    pub email: Email,
    pub password: HashedPassword,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: HashedPassword, requires_2fa: bool) -> User {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}

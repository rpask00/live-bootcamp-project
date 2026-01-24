#[derive(Clone, PartialEq, Debug)]
pub(crate) struct User {
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> User {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}

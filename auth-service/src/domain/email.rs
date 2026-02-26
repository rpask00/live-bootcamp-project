use secrecy::{ExposeSecret, SecretString};
use std::hash::Hash;
use validator::ValidationError;

#[derive(Debug, Clone)]
pub struct Email(pub(crate) SecretString);

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl Eq for Email {}
impl Email {
    pub fn parse(value: String) -> Result<Email, ValidationError> {
        if !value.contains('@') {
            return Err(ValidationError::new("Invalid email format - missing at symbol."));
        }

        if value.starts_with('@') {
            return Err(ValidationError::new("Invalid email format - missing subject."));
        }

        Ok(Email(value.into()))
    }
}

impl TryFrom<&str> for Email {
    type Error = ValidationError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value.into())
    }
}

impl AsRef<SecretString> for Email {
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::Gen;
    use rand::SeedableRng;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "test_test.com".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@example.com".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = g.size() as u64;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let email = SafeEmail().fake_with_rng(&mut rng);

            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_should_pass(email: ValidEmailFixture) {
        assert!(Email::parse(email.0).is_ok())
    }
}

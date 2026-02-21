use serde::Deserialize;
use validator::ValidationError;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn parse(value: String) -> Result<HashedPassword, ValidationError> {
        if value.len() < 8 {
            return Err(ValidationError::new("Password is to short"));
        }

        Ok(HashedPassword(value))
    }
}

impl TryFrom<&str> for HashedPassword {
    type Error = ValidationError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value.into())
    }
}

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

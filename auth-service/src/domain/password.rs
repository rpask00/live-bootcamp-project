use serde::Deserialize;
use validator::ValidationError;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn parse(value: String) -> Result<Password, ValidationError> {
        if value.len() < 8 {
            return Err(ValidationError::new("Password is to short"));
        }

        Ok(Password(value))
    }
}

impl TryFrom<&str> for Password {
    type Error = ValidationError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value.into())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}



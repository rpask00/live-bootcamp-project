use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, SecretString};
#[derive(Debug, Clone)]
pub struct HashedPassword(pub(crate) SecretString);

impl PartialEq for HashedPassword {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret() // Updated!
    }
}

impl HashedPassword {
    pub async fn parse(s: SecretString) -> Result<Self> {
        if s.expose_secret().len() < 8 {
            return Err(eyre!("Password is to short"));
        }

        match compute_password_hash(s).await {
            Ok(hashed_password) => Ok(Self(hashed_password)),
            Err(e) => Err(e),
        }
    }

    pub fn parse_password_hash(hash: SecretString) -> Result<HashedPassword, String> {
        if let Ok(hashed_string) = PasswordHash::new(hash.expose_secret().as_ref()) {
            Ok(Self(SecretString::from(hashed_string.to_string())))
        } else {
            Err(String::from("twoja stara"))
        }
    }

    #[tracing::instrument(name = "Verify raw password", skip_all)]
    pub async fn verify_raw_password(&self, password_candidate: &str) -> Result<()> {
        let current_span = tracing::Span::current();

        let password_hash = self.as_ref().to_owned();
        let password_candidate = password_candidate.to_owned();

        tokio::task::spawn_blocking(move || {
            current_span.in_scope(|| {
                let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&password_hash.expose_secret())?;
                return Argon2::default()
                    .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                    .map_err(|e| e.into());
            })
        })
        .await?
    }
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: SecretString) -> Result<SecretString> {
    let current_span = tracing::Span::current();

    tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let salt: SaltString = SaltString::generate(&mut OsRng);
            let password_hash = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(15000, 2, 1, None)?)
                .hash_password(password.expose_secret().as_bytes(), &salt)?
                .to_string();

            let bs = password_hash.into_boxed_str();
            Ok(SecretString::new(bs))
        })
    })
    .await?
}

impl AsRef<SecretString> for HashedPassword {
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::HashedPassword;

    use argon2::{
        // new
        password_hash::{rand_core::OsRng, SaltString},
        Algorithm,
        Argon2,
        Params,
        PasswordHasher,
        Version,
    };
    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;
    use quickcheck::Gen;
    use rand::SeedableRng;
    use secrecy::{ExposeSecret, SecretString};

    #[tokio::test]
    async fn empty_string_is_rejected() {
        let password = SecretString::from("");

        assert!(HashedPassword::parse(password).await.is_err());
    }

    #[tokio::test]
    async fn string_less_than_8_characters_is_rejected() {
        let password = SecretString::from("1234567");

        assert!(HashedPassword::parse(password).await.is_err());
    }

    // new
    #[test]
    fn can_parse_valid_argon2_hash() {
        // Arrange - Create a valid Argon2 hash
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(15000, 2, 1, None).unwrap());

        let hash_string = argon2.hash_password(raw_password.as_bytes(), &salt).unwrap().to_string();

        let hash_password = HashedPassword::parse_password_hash(SecretString::from(hash_string.clone())).unwrap();

        assert_eq!(hash_password.0.expose_secret(), hash_string.to_string());
        assert!(hash_password.0.expose_secret().starts_with("$argon2id$v=19$"));
    }

    // new
    #[tokio::test]
    async fn can_verify_raw_password() {
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(15000, 2, 1, None).unwrap());

        let hash_string = argon2.hash_password(raw_password.as_bytes(), &salt).unwrap().to_string();

        let hash_password = HashedPassword::parse_password_hash(SecretString::from(hash_string.clone())).unwrap();

        assert_eq!(hash_password.0.expose_secret(), hash_string.as_str());
        assert!(hash_password.0.expose_secret().starts_with("$argon2id$v=19$"));

        let result = hash_password.verify_raw_password(&raw_password).await;
        assert!(!result.is_err());

        assert_eq!(result.unwrap(), ())
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = g.size() as u64;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let password = FakePassword(8..30).fake_with_rng(&mut rng);
            Self(password)
        }
    }

    #[tokio::test]
    #[quickcheck_macros::quickcheck]
    async fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        HashedPassword::parse(SecretString::from(valid_password.0)).await.is_ok()
    }
}

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use std::error::Error;
use std::future::Future;

#[derive(Debug, Clone, PartialEq)]
pub struct HashedPassword(String); // updated!

impl HashedPassword {
    // updated!

    // TODO:
    // Update the parse function. Note that it's now async.
    // After password validation, hash the password.
    // Using the provided helper function compute_password_hash.
    pub async fn parse(s: String) -> Result<Self, String> {
        if s.len() < 8 {
            return Err(String::from("Password is to short"));
        }

        match compute_password_hash(s).await {
            Ok(hashed_password) => Ok(Self(hashed_password)),
            Err(_) => Err(String::from("Failed to hash password.")),
        }
    }

    // TODO:
    // Add a parse_password_hash function.
    // To validate the format of the hash string,
    // use PasswordHash::new
    pub fn parse_password_hash(hash: String) -> Result<HashedPassword, String> {
        if let Ok(hashed_string) = PasswordHash::new(hash.as_ref()) {
            Ok(Self(hashed_string.to_string()))
        } else {
            Err(String::from("twoja stara"))
        }
    }

    // TODO:
    // Add a verify_raw_password function.
    // To verify the password candidate use
    // Argon2::default().verify_password.
    pub async fn verify_raw_password(&self, password_candidate: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let password_hash = self.as_ref().to_owned();
        let password_candidate = password_candidate.to_owned();

        // TODO:
        // To avoid blocking other async tasks, update this function to
        // perform hashing on a separate thread pool using
        // tokio::task::spawn_blocking.
        // Return Result<(), Box<dyn Error + Send + Sync>>.
        // Every HashedPassword instance can verify a password_candidate.

        tokio::task::spawn_blocking(move || {
            let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&password_hash)?;
            return Argon2::default()
                .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                .map_err(|e| e.into());
        })
        .await?
    }
}

//..

// Helper function to hash passwords before persisting them in storage.
// TODO:
// Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking.
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    tokio::task::spawn_blocking(move || {
        let salt: SaltString = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(15000, 2, 1, None)?)
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    })
    .await?
}

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
} // updated!

#[cfg(test)]
mod tests {
    use super::HashedPassword;
    // updated!
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

    // updated!
    #[tokio::test]
    async fn empty_string_is_rejected() {
        let password = "".to_owned();

        // updated!
        assert!(HashedPassword::parse(password).await.is_err());
    }

    // updated!
    #[tokio::test]
    async fn string_less_than_8_characters_is_rejected() {
        let password = "1234567".to_owned();
        // updated!
        assert!(HashedPassword::parse(password).await.is_err());
    }

    // new
    #[test]
    fn can_parse_valid_argon2_hash() {
        // Arrange - Create a valid Argon2 hash
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );

        let hash_string = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Act
        let hash_password = HashedPassword::parse_password_hash(hash_string.clone()).unwrap();

        // Assert
        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));
    }

    // new
    #[tokio::test]
    async fn can_verify_raw_password() {
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );

        let hash_string = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let hash_password = HashedPassword::parse_password_hash(hash_string.clone()).unwrap();

        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));

        // TODO: Use verify_raw_password to verify the password match
        let result = hash_password.verify_raw_password(&raw_password).await;
        assert!(!result.is_err());

        // TODO: Assert the verification succeeds assert_eq!(result, ())
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

    // updated!
    #[tokio::test]
    #[quickcheck_macros::quickcheck]
    async fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        HashedPassword::parse(valid_password.0).await.is_ok() // updated!
    }
}

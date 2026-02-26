use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::email::Email;
use crate::domain::hashed_password::HashedPassword;
use crate::domain::user::User;
use color_eyre::eyre::eyre;
use secrecy::ExposeSecret;
use sqlx::PgPool;

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            "#,
            user.email.0.expose_secret(),
            &user.password.0.expose_secret(),
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?;

        Ok(())
    }

    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        sqlx::query!(
            r#"
                 SELECT email, password_hash, requires_2fa
                FROM users
                WHERE email = $1
            "#,
            email.0.expose_secret()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::InvalidCredentials)?
        .map(|row| {
            Ok(User::new(
                Email::parse(row.email).map_err(|e| UserStoreError::UnexpectedError(e.into()))?,
                HashedPassword::parse_password_hash(row.password_hash.into())
                    .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?,
                row.requires_2fa,
            ))
        })
        .ok_or(UserStoreError::UserNotFound)?
    }

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        let user: User = self.get_user(email).await?;
        user.password
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}

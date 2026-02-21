use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::email::Email;
use crate::domain::hashed_password::HashedPassword;
use crate::domain::user::User;
use sqlx::{Executor, PgPool};

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
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            "#,
            user.email.as_ref(),
            &user.password.as_ref(),
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::InvalidCredentials)?;

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        sqlx::query!(
            r#"
                 SELECT email, password_hash, requires_2fa
                FROM users
                WHERE email = $1
            "#,
            email.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::InvalidCredentials)?
        .map(|row| {
            Ok(User::new(
                Email::parse(row.email).map_err(|_| UserStoreError::UnexpectedError)?,
                HashedPassword::parse_password_hash(row.password_hash).map_err(|_| UserStoreError::UnexpectedError)?,
                row.requires_2fa,
            ))
        })
        .ok_or(UserStoreError::UserNotFound)?
    }

    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        let user: User = self.get_user(email).await?;
        user.password
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}

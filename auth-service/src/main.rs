use auth_service::app_state::AppState;
use auth_service::services::data_stores::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::data_stores::hashmap_user_store::HashmapUserStore;
use auth_service::services::data_stores::hashset_banned_token_store::HashsetBannedTokenStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::utils::constants::{env, prod, REDIS_HOST_NAME};
use auth_service::{get_postgres_pool, get_redis_client, Application};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient::default()));

    let app_state = AppState::new(user_store, banned_token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to start app!");
}

async fn configure_postgresql() -> PgPool {
    let database_url = std::env::var(env::DATABASE_URL).expect("DATABASE_URL must be set");

    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&database_url)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database!
    sqlx::migrate!().run(&pg_pool).await.expect("Failed to run migrations");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

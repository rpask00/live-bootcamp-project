use auth_service::app_state::AppState;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
use auth_service::services::data_stores::redis_banned_token_store::RedisBannedTokenStore;
use auth_service::services::data_stores::redis_two_fa_code_store::RedisTwoFACodeStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::utils::constants::env::DATABASE_URL_NAME;
use auth_service::utils::constants::{prod, REDIS_HOST_NAME};
use auth_service::utils::tracing::init_tracing;
use auth_service::{get_postgres_pool, get_redis_client, Application};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    init_tracing().expect("Failed to initialize tracing");
    color_eyre::install().expect("Failed to install color_eyre");

    let redis_connection = get_redis_client(REDIS_HOST_NAME.to_owned()).expect("Couldn't get Redis connection");
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_connection.get_connection().unwrap(),
    )));

    let poll = get_postgres_pool(std::env::var(DATABASE_URL_NAME).unwrap().into())
        .await
        .expect("Failed to create Postgres poll");

    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(poll)));
    // let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(
        redis_connection.get_connection().unwrap(),
    )));
    let email_client = Arc::new(RwLock::new(MockEmailClient::default()));

    let app_state = AppState::new(user_store, banned_token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to start app!");
}

#[allow(dead_code)]
fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

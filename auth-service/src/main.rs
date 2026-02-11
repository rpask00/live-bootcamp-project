use auth_service::app_state::AppState;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::services::hashset_banned_token_store::HashsetBannedTokenStore;
use auth_service::utils::constants::prod;
use auth_service::Application;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store, banned_token_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to start app!");
}

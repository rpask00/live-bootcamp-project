use crate::app_state::AppState;
use crate::domain::user::User;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let user = User::new(request.email, request.password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User signed up successfully".into(),
    });

    (StatusCode::CREATED, response)
}

#[derive(Deserialize)]
pub struct SignupRequest {
    password: String,
    email: String,
    #[serde(rename = "requires2FA")]
    requires_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

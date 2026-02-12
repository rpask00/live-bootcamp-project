use crate::app_state::AppState;
use axum::routing::post;
use axum::serve::Serve;
use axum::Router;
use dotenv::dotenv;
use reqwest::Method;
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let assets_dir = ServeDir::new("assets");

        let allowed_origins = ["http://localhost:8000".parse()?, "http://167.71.36.159:7000".parse()?];

        let cors_layer = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(allowed_origins)
            .allow_credentials(true);

        let router = Router::new()
            .fallback_service(assets_dir)
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify_token", post(routes::verify_token))
            .with_state(app_state)
            .layer(cors_layer);

        let listener = tokio::net::TcpListener::bind(address).await?;

        let address = listener.local_addr()?.to_string();

        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("Listening on {}", &self.address);
        self.server.await
    }
}

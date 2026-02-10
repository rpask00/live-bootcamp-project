use dotenv::dotenv;
use lazy_static::lazy_static;

pub mod env {
    pub const JWT_SECRET_NAME: &str = "JWT_SECRET";
    pub const JWT_COOKIE_NAME: &str = "jwt";
}

lazy_static! {
    pub static ref JWT_SECRET: String = get_jwt_secret_token();
}

fn get_jwt_secret_token() -> String {
    dotenv().ok();
    let jwt_secret = std::env::var(env::JWT_SECRET_NAME).expect("JWT_SECRET must be set");

    if jwt_secret.is_empty() {
        panic!("JWT_SECRET cannot be empty");
    }

    jwt_secret
}

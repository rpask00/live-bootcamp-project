use dotenv::dotenv;
use lazy_static::lazy_static;

pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1";

lazy_static! {
    pub static ref JWT_SECRET: String = get_jwt_secret_token();
    pub static ref REDIS_HOST_NAME: String = set_redis_host();
}

pub mod env {
    pub const JWT_SECRET_NAME: &str = "JWT_SECRET";
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const JWT_COOKIE_NAME: &str = "jwt";
    pub const REDIS_HOST_NAME_ENV_VAR: &str = "REDIS_HOST_NAME";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3001";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}

fn set_redis_host() -> String {
    dotenv().ok();
    std::env::var(env::REDIS_HOST_NAME_ENV_VAR).unwrap_or(DEFAULT_REDIS_HOSTNAME.to_owned())
}

fn get_jwt_secret_token() -> String {
    dotenv().ok();
    let jwt_secret = std::env::var(env::JWT_SECRET_NAME).expect("JWT_SECRET must be set");

    if jwt_secret.is_empty() {
        panic!("JWT_SECRET cannot be empty");
    }

    jwt_secret
}

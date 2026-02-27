use dotenv::dotenv;
use lazy_static::lazy_static;
use secrecy::SecretString;

pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1";

lazy_static! {
    pub static ref JWT_SECRET: SecretString = get_jwt_secret_token();
    pub static ref REDIS_HOST_NAME: String = set_redis_host();
    pub static ref DATABASE_URL: SecretString = set_db_url();
    pub static ref POSTMARK_AUTH_TOKEN: SecretString = set_postmark_auth_token();
}

pub mod env {
    pub const JWT_SECRET_NAME: &str = "JWT_SECRET";
    pub const DATABASE_URL_NAME: &str = "DATABASE_URL";
    pub const JWT_COOKIE_NAME: &str = "jwt";
    pub const REDIS_HOST_NAME_ENV_VAR: &str = "REDIS_HOST_NAME";
    pub const POSTMARK_AUTH_TOKEN_ENV_VAR: &str = "POSTMARK_AUTH_TOKEN";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
    pub mod email_client {
        use std::time::Duration;

        pub const BASE_URL: &str = "https://api.postmarkapp.com/email";
        // If you created your own Postmark account, make sure to use your email address!
        pub const SENDER: &str = "bogdan@codeiron.io";
        pub const TIMEOUT: Duration = std::time::Duration::from_secs(10);
    }
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
    pub mod email_client {
        use std::time::Duration;

        pub const SENDER: &str = "test@email.com";
        pub const TIMEOUT: Duration = std::time::Duration::from_millis(200);
    }
}

fn set_redis_host() -> String {
    dotenv().ok();
    std::env::var(env::REDIS_HOST_NAME_ENV_VAR).unwrap_or(DEFAULT_REDIS_HOSTNAME.to_owned())
}
fn set_db_url() -> SecretString {
    dotenv().ok();
    SecretString::from(std::env::var(env::DATABASE_URL_NAME).expect("DATABASE_URL must bet set"))
}
fn set_postmark_auth_token() -> SecretString {
    dotenv().ok();
    SecretString::from(std::env::var(env::POSTMARK_AUTH_TOKEN_ENV_VAR).expect("POSTMARK_AUTH_TOKEN must bet set"))
}

fn get_jwt_secret_token() -> SecretString {
    dotenv().ok();
    let jwt_secret = std::env::var(env::JWT_SECRET_NAME).expect("JWT_SECRET must be set");

    if jwt_secret.is_empty() {
        panic!("JWT_SECRET cannot be empty");
    }

    jwt_secret.into()
}

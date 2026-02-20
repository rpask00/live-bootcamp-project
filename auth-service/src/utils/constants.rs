use dotenv::dotenv;
use lazy_static::lazy_static;

pub mod env {
    pub const JWT_SECRET_NAME: &str = "JWT_SECRET";
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const JWT_COOKIE_NAME: &str = "jwt";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
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

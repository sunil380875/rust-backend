use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub mongo_uri: String,
    pub db_name: String,
    pub jwt_secret: String,
    pub upload_dir: String,
    pub max_upload_mb: u64,
}

impl Config {
    /// Load configuration from environment variables.
    /// Call `dotenvy::dotenv().ok()` before this.
    pub fn from_env() -> Self {
        Config {
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("PORT must be a valid u16"),
            mongo_uri: env::var("MONGO_URI")
                .expect("MONGO_URI must be set"),
            db_name: env::var("DB_NAME")
                .unwrap_or_else(|_| "rust_backend".into()),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            upload_dir: env::var("UPLOAD_DIR")
                .unwrap_or_else(|_| "./uploads".into()),
            max_upload_mb: env::var("MAX_UPLOAD_MB")
                .unwrap_or_else(|_| "10".into())
                .parse()
                .expect("MAX_UPLOAD_MB must be a valid u64"),
        }
    }
}

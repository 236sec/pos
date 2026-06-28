pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub bind_addr: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".into());
        let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".into());

        Self {
            database_url,
            redis_url,
            bind_addr,
        }
    }
}

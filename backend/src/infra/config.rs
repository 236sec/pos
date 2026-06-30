#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub bind_addr: String,
    pub head_office_url: String,
    pub jwt_secret: String,
    pub branch_id: String,
    pub auth_bypass: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".into());
        let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".into());
        let head_office_url =
            std::env::var("HEAD_OFFICE_URL").unwrap_or_else(|_| "http://localhost:9000".into());
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let branch_id = std::env::var("BRANCH_ID").expect("BRANCH_ID must be set");
        let auth_bypass = std::env::var("AUTH_BYPASS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        Self {
            database_url,
            redis_url,
            bind_addr,
            head_office_url,
            jwt_secret,
            branch_id,
            auth_bypass,
        }
    }
}

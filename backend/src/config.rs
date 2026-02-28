pub struct AppConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub rawg_api_key: String,
    pub frontend_url: String,
    // Zitadel OIDC (PKCE auth code flow)
    pub zitadel_issuer: String,
    pub zitadel_client_id: String,
    pub zitadel_redirect_uri: String,
    pub zitadel_post_logout_uri: String,
    pub session_secret: String,
    pub redis_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "8090".into())
            .parse()
            .expect("PORT must be a number");
        let frontend_url =
            std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:8080".into());
        let backend_url = std::env::var("BACKEND_URL")
            .unwrap_or_else(|_| format!("http://localhost:{}", port));

        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/gamereview".into()),
            host,
            port,
            rawg_api_key: std::env::var("RAWG_API_KEY").unwrap_or_default(),
            frontend_url,
            zitadel_issuer: std::env::var("ZITADEL_ISSUER")
                .unwrap_or_else(|_| "http://localhost:8080".into()),
            zitadel_client_id: std::env::var("ZITADEL_CLIENT_ID")
                .unwrap_or_else(|_| "gamereview-app".into()),
            zitadel_redirect_uri: std::env::var("ZITADEL_REDIRECT_URI")
                .unwrap_or_else(|_| format!("{}/api/auth/callback", backend_url)),
            zitadel_post_logout_uri: std::env::var("ZITADEL_POST_LOGOUT_URI")
                .unwrap_or_else(|_| frontend_url.clone()),
            session_secret: std::env::var("SESSION_SECRET").unwrap_or_else(|_| {
                "super-secret-key-change-in-production-must-be-at-least-64-bytes-long-for-security!!"
                    .into()
            }),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".into()),
        }
    }
}

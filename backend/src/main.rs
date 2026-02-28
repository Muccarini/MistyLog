use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, IssuerUrl, RedirectUrl,
};
use sea_orm::Database;
use std::sync::Arc;

mod config;
mod errors;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_default_env();

    let cfg = config::AppConfig::from_env();
    let db = Database::connect(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    migration::run_migrations(&db)
        .await
        .expect("Failed to run migrations");

    log::info!("Starting server at {}:{}", cfg.host, cfg.port);

    // Discover Zitadel OIDC provider metadata
    let issuer_url = IssuerUrl::new(cfg.zitadel_issuer.clone())
        .expect("Invalid ZITADEL_ISSUER URL");
    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .expect("Failed to discover Zitadel OIDC metadata");

    let oidc_client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(cfg.zitadel_client_id.clone()),
        None, // No client secret for PKCE public clients
    )
    .set_redirect_uri(
        RedirectUrl::new(cfg.zitadel_redirect_uri.clone())
            .expect("Invalid ZITADEL_REDIRECT_URI"),
    );

    let oidc_client = Arc::new(oidc_client);
    let cfg = Arc::new(cfg);

    let secret_key = Key::from(cfg.session_secret.as_bytes());
    let rawg_api_key = cfg.rawg_api_key.clone();
    let bind_addr = format!("{}:{}", cfg.host, cfg.port);

    // Connect to Redis for session storage
    let redis_store = RedisSessionStore::new(&cfg.redis_url)
        .await
        .expect("Failed to connect to Redis");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.frontend_url)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_secure(false) // Set to true in production with HTTPS
                    .cookie_http_only(true)
                    .cookie_same_site(actix_web::cookie::SameSite::Lax)
                    .build(),
            )
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::from(oidc_client.clone()))
            .app_data(web::Data::from(cfg.clone()))
            .app_data(web::Data::new(services::rawg::RawgService::new(
                rawg_api_key.clone(),
            )))
            .configure(routes::configure)
    })
    .bind(&bind_addr)?
    .run()
    .await
}

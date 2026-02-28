use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use openidconnect::{
    core::CoreClient,
    ClientId, RedirectUrl,
};
use sea_orm::Database;

mod config;
mod errors;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod seeds;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let cfg = config::AppConfig::from_env();
    let db = Database::connect(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    migration::run_migrations(&db)
        .await
        .expect("Failed to run migrations");

    log::info!("Starting server at {}:{}", cfg.host, cfg.port);

    // TODO: Implement proper OIDC client with Zitadel discovery
    // For now, use a stub to allow the server to start
    let oidc_client: CoreClient = panic!("OIDC not yet implemented");
    // This will be replaced with proper OIDC setup

    let oidc_client_data = web::Data::new(oidc_client);
    let cfg_data = web::Data::new(cfg.clone());

    let secret_key = Key::from(cfg.session_secret.as_bytes());
    let rawg_api_key = cfg.rawg_api_key.clone();
    let frontend_url = cfg.frontend_url.clone();
    let redis_url = cfg.redis_url.clone();
    let bind_addr = format!("{}:{}", cfg.host, cfg.port);

    // Connect to Redis for session storage
    let redis_store = RedisSessionStore::new(&redis_url)
        .await
        .expect("Failed to connect to Redis");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
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
            .app_data(web::Data::clone(&oidc_client_data))
            .app_data(web::Data::clone(&cfg_data))
            .app_data(web::Data::new(services::rawg::RawgService::new(
                rawg_api_key.clone(),
            )))
            .configure(routes::configure)
    })
    .bind(&bind_addr)?
    .run()
    .await
}

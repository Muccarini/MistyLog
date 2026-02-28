use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::get().to(handlers::auth::login))
            .route("/callback", web::get().to(handlers::auth::callback))
            .route("/me", web::get().to(handlers::auth::me))
            .route("/logout", web::post().to(handlers::auth::logout)),
    );
}

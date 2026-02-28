use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reviews")
            .route("/{id}", web::put().to(handlers::reviews::update_review))
            .route("/{id}", web::delete().to(handlers::reviews::delete_review)),
    );
}

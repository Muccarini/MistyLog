use actix_web::web;

mod auth;
mod games;
mod reviews;
mod seed;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::configure)
            .configure(games::configure)
            .configure(reviews::configure)
            .configure(seed::configure),
    );
}

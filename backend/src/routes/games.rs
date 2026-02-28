use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/games")
            .route("", web::get().to(handlers::games::list_games))
            .route("", web::post().to(handlers::games::create_game))
            .route("/search/rawg", web::get().to(handlers::games::search_rawg))
            .route("/{id}", web::get().to(handlers::games::get_game))
            .route("/{id}", web::put().to(handlers::games::update_game))
            .route("/{id}", web::delete().to(handlers::games::delete_game))
            .route(
                "/{id}/reviews",
                web::get().to(handlers::reviews::list_reviews_for_game),
            )
            .route(
                "/{id}/reviews",
                web::post().to(handlers::reviews::create_review),
            ),
    );
}

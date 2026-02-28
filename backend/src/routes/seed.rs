use actix_web::{web, HttpResponse};
use sea_orm::DbConn;
use crate::seeds;

pub async fn seed_db(db: web::Data<DbConn>) -> HttpResponse {
    match seeds::seed_mock_data(db.as_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Database seeded with mock data successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to seed database: {}", e)
        })),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/seed", web::post().to(seed_db));
}

use actix_web::{web, HttpResponse};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::game::{self, CreateGameRequest, GameListResponse, GameQuery, UpdateGameRequest};
use crate::services::rawg::RawgService;

fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub async fn list_games(
    db: web::Data<DatabaseConnection>,
    query: web::Query<GameQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);

    let mut select = game::Entity::find();

    if let Some(ref search) = query.search {
        select = select.filter(game::Column::Title.contains(search));
    }
    if let Some(ref genre) = query.genre {
        select = select.filter(game::Column::Genre.eq(genre));
    }
    if let Some(ref platform) = query.platform {
        select = select.filter(game::Column::Platform.contains(platform));
    }

    let total = select.clone().count(db.get_ref()).await?;
    let games = select
        .order_by_desc(game::Column::CreatedAt)
        .paginate(db.get_ref(), per_page)
        .fetch_page(page - 1)
        .await?;

    Ok(HttpResponse::Ok().json(GameListResponse {
        games,
        total,
        page,
        per_page,
    }))
}

pub async fn get_game(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let game_id = path.into_inner();
    let game = game::Entity::find_by_id(game_id)
        .one(db.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Game not found".into()))?;

    Ok(HttpResponse::Ok().json(game))
}

pub async fn create_game(
    db: web::Data<DatabaseConnection>,
    _auth: AuthUser,
    body: web::Json<CreateGameRequest>,
) -> Result<HttpResponse, AppError> {

    if body.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title is required".into()));
    }

    let now = chrono::Utc::now().naive_utc();
    let slug = slugify(&body.title);

    let new_game = game::ActiveModel {
        title: Set(body.title.trim().to_string()),
        slug: Set(slug),
        description: Set(body.description.clone()),
        genre: Set(body.genre.clone()),
        platform: Set(body.platform.clone()),
        release_date: Set(body.release_date),
        cover_image_url: Set(body.cover_image_url.clone()),
        rawg_id: Set(body.rawg_id),
        avg_rating: Set(None),
        review_count: Set(0),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let result = new_game.insert(db.get_ref()).await?;
    Ok(HttpResponse::Created().json(result))
}

pub async fn update_game(
    db: web::Data<DatabaseConnection>,
    _auth: AuthUser,
    path: web::Path<i32>,
    body: web::Json<UpdateGameRequest>,
) -> Result<HttpResponse, AppError> {

    let game_id = path.into_inner();
    let game = game::Entity::find_by_id(game_id)
        .one(db.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Game not found".into()))?;

    let mut active: game::ActiveModel = game.into();
    let now = chrono::Utc::now().naive_utc();

    if let Some(ref title) = body.title {
        active.title = Set(title.trim().to_string());
        active.slug = Set(slugify(title));
    }
    if let Some(ref description) = body.description {
        active.description = Set(Some(description.clone()));
    }
    if let Some(ref genre) = body.genre {
        active.genre = Set(Some(genre.clone()));
    }
    if let Some(ref platform) = body.platform {
        active.platform = Set(Some(platform.clone()));
    }
    if let Some(ref release_date) = body.release_date {
        active.release_date = Set(Some(*release_date));
    }
    if let Some(ref cover_image_url) = body.cover_image_url {
        active.cover_image_url = Set(Some(cover_image_url.clone()));
    }
    active.updated_at = Set(now);

    let result = active.update(db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_game(
    db: web::Data<DatabaseConnection>,
    _auth: AuthUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {

    let game_id = path.into_inner();
    let result = game::Entity::delete_by_id(game_id)
        .exec(db.get_ref())
        .await?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound("Game not found".into()));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Game deleted" })))
}

pub async fn search_rawg(
    rawg: web::Data<RawgService>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    let search = query
        .get("q")
        .ok_or_else(|| AppError::BadRequest("Query parameter 'q' is required".into()))?;

    let results = rawg
        .search_games(search)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(HttpResponse::Ok().json(results))
}

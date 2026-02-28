use actix_web::{web, HttpResponse};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::game;
use crate::models::review::{self, CreateReviewRequest, ReviewWithUser, UpdateReviewRequest};
use crate::models::user;

/// Recalculate the average rating for a game and persist it.
async fn update_game_rating(db: &DatabaseConnection, game_id: i32) -> Result<(), AppError> {
    let reviews = review::Entity::find()
        .filter(review::Column::GameId.eq(game_id))
        .all(db)
        .await?;

    let count = reviews.len() as i32;
    let avg = if count > 0 {
        let sum: f64 = reviews.iter().map(|r| r.rating as f64).sum();
        Some(sum / count as f64)
    } else {
        None
    };

    let game = game::Entity::find_by_id(game_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Game not found".into()))?;

    let mut active: game::ActiveModel = game.into();
    active.avg_rating = Set(avg);
    active.review_count = Set(count);
    active.update(db).await?;

    Ok(())
}

pub async fn list_reviews_for_game(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let game_id = path.into_inner();

    // Verify game exists
    game::Entity::find_by_id(game_id)
        .one(db.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Game not found".into()))?;

    let reviews = review::Entity::find()
        .filter(review::Column::GameId.eq(game_id))
        .order_by_desc(review::Column::CreatedAt)
        .all(db.get_ref())
        .await?;

    // Enrich with user info
    let mut enriched: Vec<ReviewWithUser> = Vec::new();
    for r in reviews {
        let u = user::Entity::find_by_id(r.user_id)
            .one(db.get_ref())
            .await?;
        let (username, display_name) = match u {
            Some(u) => (u.username, u.display_name),
            None => ("deleted".into(), None),
        };
        enriched.push(ReviewWithUser {
            id: r.id,
            user_id: r.user_id,
            game_id: r.game_id,
            rating: r.rating,
            title: r.title,
            body: r.body,
            created_at: r.created_at,
            updated_at: r.updated_at,
            username,
            display_name,
        });
    }

    Ok(HttpResponse::Ok().json(enriched))
}

pub async fn create_review(
    db: web::Data<DatabaseConnection>,
    auth: AuthUser,
    path: web::Path<i32>,
    body: web::Json<CreateReviewRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = auth.user_id;

    let game_id = path.into_inner();

    // Verify game exists
    game::Entity::find_by_id(game_id)
        .one(db.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Game not found".into()))?;

    // Validate rating
    if body.rating < 1 || body.rating > 10 {
        return Err(AppError::BadRequest(
            "Rating must be between 1 and 10".into(),
        ));
    }

    // Check if user already reviewed this game
    let existing = review::Entity::find()
        .filter(
            review::Column::UserId
                .eq(user_id)
                .and(review::Column::GameId.eq(game_id)),
        )
        .one(db.get_ref())
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest(
            "You have already reviewed this game".into(),
        ));
    }

    let now = chrono::Utc::now().naive_utc();

    let new_review = review::ActiveModel {
        user_id: Set(user_id),
        game_id: Set(game_id),
        rating: Set(body.rating),
        title: Set(body.title.clone()),
        body: Set(body.body.clone()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let result = new_review.insert(db.get_ref()).await?;

    // Update game average rating
    update_game_rating(db.get_ref(), game_id).await?;

    Ok(HttpResponse::Created().json(result))
}

pub async fn update_review(
    db: web::Data<DatabaseConnection>,
    auth: AuthUser,
    path: web::Path<i32>,
    body: web::Json<UpdateReviewRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = auth.user_id;

    let review_id = path.into_inner();
    let review = review::Entity::find_by_id(review_id)
        .one(db.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Review not found".into()))?;

    if review.user_id != user_id {
        return Err(AppError::Forbidden);
    }

    let game_id = review.game_id;
    let mut active: review::ActiveModel = review.into();
    let now = chrono::Utc::now().naive_utc();

    if let Some(rating) = body.rating {
        if rating < 1 || rating > 10 {
            return Err(AppError::BadRequest(
                "Rating must be between 1 and 10".into(),
            ));
        }
        active.rating = Set(rating);
    }
    if let Some(ref title) = body.title {
        active.title = Set(Some(title.clone()));
    }
    if let Some(ref body_text) = body.body {
        active.body = Set(body_text.clone());
    }
    active.updated_at = Set(now);

    let result = active.update(db.get_ref()).await?;

    // Update game average rating
    update_game_rating(db.get_ref(), game_id).await?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_review(
    db: web::Data<DatabaseConnection>,
    auth: AuthUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = auth.user_id;

    let review_id = path.into_inner();
    let review = review::Entity::find_by_id(review_id)
        .one(db.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Review not found".into()))?;

    if review.user_id != user_id {
        return Err(AppError::Forbidden);
    }

    let game_id = review.game_id;
    review::Entity::delete_by_id(review_id)
        .exec(db.get_ref())
        .await?;

    // Update game average rating
    update_game_rating(db.get_ref(), game_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Review deleted" })))
}

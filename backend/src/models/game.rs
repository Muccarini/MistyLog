use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub platform: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
    pub cover_image_url: Option<String>,
    pub rawg_id: Option<i32>,
    pub avg_rating: Option<f64>,
    pub review_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::review::Entity")]
    Reviews,
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reviews.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    pub title: String,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub platform: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
    pub cover_image_url: Option<String>,
    pub rawg_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGameRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub platform: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
    pub cover_image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GameQuery {
    pub search: Option<String>,
    pub genre: Option<String>,
    pub platform: Option<String>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct GameListResponse {
    pub games: Vec<Model>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

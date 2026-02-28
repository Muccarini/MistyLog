use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// Zitadel subject ID (unique external identity)
    #[sea_orm(unique)]
    pub sub: String,
    #[sea_orm(unique)]
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
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

/// DTO for public user info
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

impl From<Model> for UserResponse {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            username: m.username,
            email: m.email,
            display_name: m.display_name,
            avatar_url: m.avatar_url,
            created_at: m.created_at,
        }
    }
}

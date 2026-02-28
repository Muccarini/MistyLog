use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reviews::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reviews::UserId).integer().not_null())
                    .col(ColumnDef::new(Reviews::GameId).integer().not_null())
                    .col(ColumnDef::new(Reviews::Rating).small_integer().not_null())
                    .col(ColumnDef::new(Reviews::Title).string_len(255).null())
                    .col(ColumnDef::new(Reviews::Body).text().not_null())
                    .col(
                        ColumnDef::new(Reviews::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Reviews::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reviews_user")
                            .from(Reviews::Table, Reviews::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reviews_game")
                            .from(Reviews::Table, Reviews::GameId)
                            .to(Games::Table, Games::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // One review per user per game
        manager
            .create_index(
                Index::create()
                    .name("idx_reviews_user_game")
                    .table(Reviews::Table)
                    .col(Reviews::UserId)
                    .col(Reviews::GameId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Fast lookup of reviews by game
        manager
            .create_index(
                Index::create()
                    .name("idx_reviews_game_id")
                    .table(Reviews::Table)
                    .col(Reviews::GameId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reviews::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reviews {
    Table,
    Id,
    UserId,
    GameId,
    Rating,
    Title,
    Body,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Games {
    Table,
    Id,
}

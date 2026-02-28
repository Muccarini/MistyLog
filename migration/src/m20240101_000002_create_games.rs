use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Games::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Games::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Games::Title).string_len(500).not_null())
                    .col(
                        ColumnDef::new(Games::Slug)
                            .string_len(500)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Games::Description).text().null())
                    .col(ColumnDef::new(Games::Genre).string_len(100).null())
                    .col(ColumnDef::new(Games::Platform).string_len(255).null())
                    .col(ColumnDef::new(Games::ReleaseDate).date().null())
                    .col(ColumnDef::new(Games::CoverImageUrl).text().null())
                    .col(ColumnDef::new(Games::RawgId).integer().null().unique_key())
                    .col(
                        ColumnDef::new(Games::AvgRating)
                            .double()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Games::ReviewCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Games::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Games::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Games::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Games {
    Table,
    Id,
    Title,
    Slug,
    Description,
    Genre,
    Platform,
    ReleaseDate,
    CoverImageUrl,
    RawgId,
    AvgRating,
    ReviewCount,
    CreatedAt,
    UpdatedAt,
}

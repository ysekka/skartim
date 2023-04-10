use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(
            Type::create()
            .as_enum(ArticleType::Type)
            .values([
                ArticleType::Announcement,
                ArticleType::FullPage,
                ArticleType::Normal
            ])
            .to_owned()
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_type(
            Type::drop()
            .if_exists()
            .name(ArticleType::Type)
            .to_owned()
        )
        .await
    }
}

#[derive(Iden)]
pub enum ArticleType {
    #[iden = "article_type"]
    Type,

    #[iden = "NORMAL"]
    Normal,

    #[iden = "FULLPAGE"]
    FullPage,

    #[iden = "ANNOUNCEMENT"]
    Announcement,
}
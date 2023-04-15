use sea_orm_migration::prelude::*;

use super::article_type::ArticleType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
            .if_not_exists()
            .table(ArticlesTable::Table)
            .col(
                ColumnDef::new(ArticlesTable::ArticleUuid)
                .default(Func::cust(Alias::new("gen_random_uuid")))
                .primary_key()
                .unique_key()
                .not_null()
                .uuid()
            )
            .col(
                ColumnDef::new(ArticlesTable::ArticleTimestamp)
                .default(Expr::current_timestamp())
                .date_time()
                .not_null()
            )
            .col(
                ColumnDef::new(ArticlesTable::ArticleThumbnail)
                .string()
            )
            .col(
                ColumnDef::new(ArticlesTable::ArticleType)
                .custom(ArticleType::Type)
                .not_null()
            )
            .col(
                ColumnDef::new(ArticlesTable::ArticleViews)
                .big_integer()
                .default(0)
                .not_null()
            )
            .col(
                ColumnDef::new(ArticlesTable::ArticleTags)
                .array(ColumnType::String(None))
            )
            .col(
                ColumnDef::new(ArticlesTable::ArticleVisibility)
                .default(true)
                .not_null()
                .boolean()
            )
            .col(
                ColumnDef::new(ArticlesTable::ArticleAuthor)
                .not_null()
                .uuid()
            )
            .col(
                ColumnDef::new(ArticlesTable::ArticleTitle)
                .not_null()
                .string()
            )
            .col(
                ColumnDef::new(ArticlesTable::ArticleContent)
                .not_null()
                .string()
            )
            .foreign_key(
                ForeignKey::create()
                .from(ArticlesTable::Table, ArticlesTable::ArticleAuthor)
                .to(crate::users_table::UsersTable::Table, crate::users_table::UsersTable::UserUuid)
            )
            .to_owned()
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop()
            .if_exists()
            .table(ArticlesTable::Table)
            .to_owned()
        )
        .await
    }
}

#[derive(Iden)]
pub enum ArticlesTable {
    Table,

    #[iden = "article_uuid"]
    ArticleUuid,

    #[iden = "article_thumbnail"]
    ArticleThumbnail,

    #[iden = "article_timestamp"]
    ArticleTimestamp,

    #[iden = "article_type"]
    ArticleType,

    #[iden = "article_views"]
    ArticleViews,

    #[iden = "article_visibility"]
    ArticleVisibility,

    #[iden = "article_tags"]
    ArticleTags,

    #[iden = "article_author"]
    ArticleAuthor,

    #[iden = "article_title"]
    ArticleTitle,

    #[iden = "article_content"]
    ArticleContent,
}
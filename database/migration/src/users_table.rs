use sea_orm_migration::prelude::*;

use super::user_status::UserStatus;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
            .if_not_exists()
            .table(UsersTable::Table)
            .col(
                ColumnDef::new(UsersTable::UserUuid)
                .default(Func::cust(Alias::new("gen_random_uuid")))
                .primary_key()
                .unique_key()
                .not_null()
                .uuid()
            )
            .col(
                ColumnDef::new(UsersTable::UserRealname)
                .not_null()
                .string()
            )
            .col(
                ColumnDef::new(UsersTable::UserEmail)
                .not_null()
                .string()
            )
            .col(
                ColumnDef::new(UsersTable::UserStatus)
                .custom(UserStatus::Type)
                .not_null()
            )
            .col(
                ColumnDef::new(UsersTable::UserPassword)
                .binary_len(256)
                .not_null()
                .binary()
            )
            .to_owned()
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop()
            .if_exists()
            .table(UsersTable::Table)
            .to_owned()
        )
        .await
    }
}

#[derive(Iden)]
pub enum UsersTable {
    Table,

    #[iden = "user_uuid"]
    UserUuid,

    #[iden = "user_realname"]
    UserRealname,

    #[iden = "user_lastlogin"]
    UserLastLogin,

    #[iden = "user_email"]
    UserEmail,

    #[iden = "user_status"]
    UserStatus,

    #[iden = "user_password"]
    UserPassword,
}
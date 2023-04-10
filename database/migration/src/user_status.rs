use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(
            Type::create()
            .as_enum(UserStatus::Type)
            .values([
                UserStatus::Administrator,
                UserStatus::CoAdministrator,
                UserStatus::HighAuthor,
                UserStatus::Author
            ])
            .to_owned()
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_type(
            Type::drop()
            .if_exists()
            .name(UserStatus::Type)
            .to_owned()
        )
        .await
    }
}

#[derive(Iden)]
pub enum UserStatus {
    #[iden = "user_status"]
    Type,

    #[iden = "ADMINISTRATOR"]
    Administrator,

    #[iden = "COADMINISTRATOR"]
    CoAdministrator,

    #[iden = "HIGHAUTHOR"]
    HighAuthor,

    #[iden = "AUTHOR"]
    Author,
}
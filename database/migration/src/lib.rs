pub use sea_orm_migration::prelude::*;

pub mod articles_table;
pub mod article_type;
pub mod user_status;
pub mod users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(user_status::Migration),
            Box::new(article_type::Migration),
            Box::new(users_table::Migration),
            Box::new(articles_table::Migration),
        ]
    }
}

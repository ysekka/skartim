use entity::sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub database_connection: DatabaseConnection,
    pub public_directory: std::path::PathBuf,
}
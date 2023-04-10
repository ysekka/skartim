pub mod commands;
pub mod settings;
pub mod paths;
pub mod saves;

pub mod jwt_claims {
    #[derive(Clone, serde::Deserialize, serde::Serialize)]
    pub struct Claims {
        pub user_uuid: uuid::Uuid,
        pub user_password: String,
    }
}
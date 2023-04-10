#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Saves {
    pub server: Option<url::Url>,
    pub login: Option<LoginInfo>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LoginInfo {
    pub user_uuid: uuid::Uuid,
    pub user_password: String,
    pub daily_password: String,
}
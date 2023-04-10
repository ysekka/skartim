use super::server::ServerSettings;
use super::smtp::SmtpSettings;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub server: Vec<ServerSettings>,
    pub database_url: url::Url,
    pub smtp: SmtpSettings,

    pub public_directory: std::path::PathBuf,
    pub administrator_email: String,
}

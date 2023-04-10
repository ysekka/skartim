#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SmtpSettings {
    pub smtp_address: String,
    pub smtp_username: String,
    pub smtp_password: String,
}
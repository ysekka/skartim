#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TlsSslSettings {
    pub ssl_key: std::path::PathBuf,
    pub ssl_cert: std::path::PathBuf,
}
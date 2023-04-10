use super::tlsssl::TlsSslSettings;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerSettings {
    pub server_host: std::net::IpAddr,
    pub server_port: u16,

    pub tls: Option<TlsSslSettings>,
}
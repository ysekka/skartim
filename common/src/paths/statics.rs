lazy_static::lazy_static!(
    pub static ref SETTINGS_PATH: std::path::PathBuf = std::path::Path::new(env!("HOME"))
    .join(".config")
    .join("skarticle")
    .join("config.ron");

    pub static ref PASSWORD_PATH: std::path::PathBuf = std::path::Path::new(env!("HOME"))
    .join(".cache")
    .join("skarticle")
    .join("password.bin");

    pub static ref SAVES_PATH: std::path::PathBuf = std::path::Path::new(env!("HOME"))
    .join(".cache")
    .join("skadmin")
    .join("saves.toml");
);
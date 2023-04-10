pub fn init() {
    if let Some(parent_path) = super::statics::SETTINGS_PATH.parent() {
        log::info!("Creating directories which in relation with settings file.");

        std::fs::create_dir_all(parent_path)
        .unwrap_or_else(|error| {
            log::error!("Error occured during creating directories which in relation with settings file.");
            log::error!("{error}");

            std::process::exit(1)
        });
    }

    if let Some(parent_path) = super::statics::PASSWORD_PATH.parent() {
        log::info!("Creating directories which in relation with password file.");

        std::fs::create_dir_all(parent_path)
        .unwrap_or_else(|error| {
            log::error!("Error occured during creating directories which in relation with password file.");
            log::error!("{error}");

            std::process::exit(1)
        });
    }

    if !super::statics::SETTINGS_PATH.exists() {
        log::info!("Creating settings file.");

        std::fs::File::create(super::statics::SETTINGS_PATH.as_path())
        .unwrap_or_else(|error| {
            log::error!("Error occured during creating settings file.");
            log::error!("{error}");

            std::process::exit(1)
        });
    }

    if !super::statics::PASSWORD_PATH.exists() {
        log::info!("Creating password file.");

        std::fs::File::create(super::statics::PASSWORD_PATH.as_path())
        .unwrap_or_else(|error| {
            log::error!("Error occured during creating password file.");
            log::error!("{error}");

            std::process::exit(1)
        });
    }
}

pub fn init_skadmin() {
    if let Some(parent) = super::statics::SAVES_PATH.parent() {
        std::fs::create_dir_all(parent)
        .unwrap_or_else(|error| {
            log::error!("Error occured during creating parent paths of saves path. error: [{error}]");
            std::process::exit(exitcode::IOERR)
        });
    }

    if !super::statics::SAVES_PATH.exists() {
        std::fs::File::create(super::statics::SAVES_PATH.as_path())
        .unwrap_or_else(|error| {
            log::error!("Error occured during creating saves file. error: [{error}]");
            std::process::exit(exitcode::IOERR);
        });
    }
}
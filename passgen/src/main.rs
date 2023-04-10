use env_logger as el;

use common::settings::r#struct as stcSettings;
use entity::sea_orm::{Database, EntityTrait};
use lettre::Transport;
use rand::Rng;
use sha2::Digest;
use actix_rt as ar;

use common::paths::statics::{
    PASSWORD_PATH,
    SETTINGS_PATH,
};

#[ar::main]
async fn main() {
    el::init();
    common::paths::initialization::init();

    log::info!("Reading settings file.");

    let settings_content = std::fs::read_to_string(SETTINGS_PATH.as_path())
    .unwrap_or_else(|error| {
        log::error!("Error occured during reading settings file.");
        log::error!("{error}");
        std::process::exit(1)
    });

    let settings = ron::from_str::<stcSettings::Settings>(&settings_content)
    .unwrap_or_else(|error| {
        log::error!("Error occured during parsing settings content.");
        log::error!("{error}");
        std::process::exit(1)
    });

    let database_connection = Database::connect(settings.database_url.as_str()).await
    .unwrap_or_else(|error| {
        log::error!("Error occured during establishing database connection.");
        log::error!("{error}");
        std::process::exit(1)
    });

    log::info!("Initialing SMTP transporter.");

    let mailer = lettre::SmtpTransport::starttls_relay(&settings.smtp.smtp_address.to_string())
    .unwrap_or_else(|error| {
        log::error!("Error occured during smtp transporter.");
        log::error!("{error}");
        std::process::exit(1)
    })
    .credentials(lettre::transport::smtp::authentication::Credentials::new(
        settings.smtp.smtp_username.to_owned(),
        settings.smtp.smtp_password,
    ))
    .build();

    log::info!("Testing connection.");

    let test_connection = mailer.test_connection()
    .unwrap_or_else(|error| {
        log::error!("Could not testing connection.");
        log::error!("{error:#?}");
        std::process::exit(1)
    });

    if test_connection {
        let sender_address = settings.smtp.smtp_username
        .parse::<lettre::Address>()
        .unwrap_or_else(|error| {
            log::error!("SMTP username is not in email address format.");
            log::error!("{error}");
            std::process::exit(1)
        });

        loop {
            let random_number = rand::thread_rng().gen::<usize>();

            let mut hasher = sha2::Sha256::new();

            hasher.update(random_number.to_string());

            let hashed_pass = hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect::<String>();

            log::info!("Generated password: {hashed_pass:?}");

            std::fs::write(PASSWORD_PATH.as_path(), hashed_pass.to_owned())
            .unwrap_or_else(|error| {
                log::error!("Error occured during writing hashed password over password path.");
                log::error!("{error}");
                std::process::exit(1)
            });

            let users_query = entity::users_table::Entity::find()
            .all(&database_connection)
            .await;

            let users = users_query
            .unwrap_or_else(|error| {
                log::error!("Sql Error: ({error})");
                std::process::exit(1)
            });

            for user in users.iter() {
                let user_address =  user.user_email
                .parse::<lettre::Address>()
                .unwrap_or_else(|error| {
                    log::error!("[{}] user`s email address is not in correct format.", user.user_uuid);
                    log::error!("[{}] {error}", user.user_uuid);
                    std::process::exit(1)
                });

                let message = lettre::Message::builder()
                .from(lettre::message::Mailbox::new(Some("PassGen".to_owned()), sender_address.to_owned()))
                .subject("Generated Daily Password")
                .to(lettre::message::Mailbox::new(None, user_address))
                .body(hashed_pass.to_owned())
                .expect("Error occured during creating of message.");

                mailer.send(&message)
                .unwrap_or_else(|error| {
                    log::error!("[{}] Error could not sent.", user.user_uuid);
                    log::error!("[{}] {error}", user.user_uuid);
                    std::process::exit(1)
                });

                log::info!("Email sent to [{}].", user.user_uuid);
            }

            std::thread::sleep(std::time::Duration::from_secs(60 * 60 * 24))
        }
    }

    log::error!("Connection failed.");
}
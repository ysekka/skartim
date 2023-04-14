use actix_web_lab as awl;
use env_logger as el;
use actix_cors as ac;
use actix_web as aw;

use entity::sea_orm::Database;

use common::paths::statics::SETTINGS_PATH;
use lettre::Transport;
use sha2::Digest;

pub mod routes;
pub mod state;

#[aw::main]
async fn main() -> std::io::Result<()> {
    use crate::routes::catcher::client_catcher;
    use crate::routes::configure;
    use crate::state::r#struct as stcState;
    use common::settings::r#struct as stcSettings;
    use migration::MigratorTrait;

    el::init_from_env(el::Env::default().default_filter_or("TRACE"));
    common::paths::initialization::init();

    log::info!("Reading settings file.");

    let settings_content =
        std::fs::read_to_string(SETTINGS_PATH.as_path()).unwrap_or_else(|error| {
            log::error!("Error occured during reading settings file.");
            log::error!("{error}");
            std::process::exit(1)
        });

    let settings =
        ron::from_str::<stcSettings::Settings>(&settings_content).unwrap_or_else(|error| {
            log::error!("Error occured during parsing settings content.");
            log::error!("{error}");
            std::process::exit(1)
        });

    let database_connection = Database::connect(settings.database_url.as_str())
        .await
        .unwrap_or_else(|error| {
            log::error!("Error occured during database connection.");
            log::error!("{error}");
            std::process::exit(1)
        });

    migration::Migrator::up(&database_connection, None)
        .await
        .unwrap_or_else(|error| {
            log::error!("Error occured during migrating up.");
            log::error!("{error}");
            std::process::exit(1)
        });

    let app_state = stcState::AppState {
        database_connection,
        public_directory: settings.public_directory.clone()
    };

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

    if !test_connection {
        log::warn!("Mailer could not establish a connection to the SMTP Server. Error reporting system may not work.")
    }

    let mut server = aw::HttpServer::new(move || {
        let administrator_email = settings.administrator_email.clone();
        let sender_email = settings.smtp.smtp_username.clone();
        let mailer = mailer.clone();

        aw::App::new()
            .app_data(aw::web::Data::new(app_state.clone()))
            .app_data(aw::web::PayloadConfig::new(2147483648 /* 2 GIGABYTE */))
            .wrap(awl::middleware::PanicReporter::new(move |_| {
                let mut hasher = sha2::Sha256::new();
                hasher.update(rand::random::<usize>().to_string());
                let hashed_error_point_code = hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect::<String>();
                log::error!("[PANIC !!!][{:?}]", hashed_error_point_code);

                let sender_address = sender_email
                .parse::<lettre::Address>()
                .unwrap_or_else(|error| {
                    log::error!("SMTP username is not in email address format.");
                    log::error!("{error}");
                    std::process::exit(1)
                });

                let administrator_email =  administrator_email
                .parse::<lettre::Address>()
                .unwrap_or_else(|_| {
                    log::error!("Administrator`s email address is not in correct format.");
                    std::process::exit(1)
                });

                let message = lettre::Message::builder()
                .from(lettre::message::Mailbox::new(Some("Skarticle Error Reporting System".to_owned()), sender_address))
                .subject("INTERNAL ERROR OCCURED")
                .to(lettre::message::Mailbox::new(None, administrator_email))
                .body(format!("[PANIC !!! We need help !!!][{}]", hashed_error_point_code))
                .expect("Error occured during creating of message.");

                mailer.send(&message)
                .unwrap_or_else(|_| {
                    log::error!("Error could not sent.");
                    std::process::exit(1)
                });
            }))
            .wrap(aw::middleware::ErrorHandlers::new().default_handler_client(client_catcher))
            .wrap(ac::Cors::permissive())
            .configure(configure)
    });

    for (index, server_settings) in settings.server.iter().enumerate() {
        match &server_settings.tls {
            Some(config) => {
                let mut acceptor_builder =
                    openssl::ssl::SslAcceptor::mozilla_intermediate(openssl::ssl::SslMethod::tls())
                        .unwrap_or_else(|error| {
                            log::error!("Tls/Ssl acceptor builder cannot be initialized.");
                            log::error!("{error}");
                            std::process::exit(1)
                        });

                acceptor_builder
                    .set_private_key_file(&config.ssl_key, openssl::ssl::SslFiletype::PEM)
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during setting private key file.");
                        log::error!("{error}");
                        std::process::exit(1)
                    });

                acceptor_builder
                    .set_certificate_chain_file(&config.ssl_cert)
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during setting private certificate chain file.");
                        log::error!("{error}");
                        std::process::exit(1)
                    });

                server = server
                    .bind_openssl(
                        (server_settings.server_host, server_settings.server_port),
                        acceptor_builder,
                    )
                    .unwrap_or_else(|error| {
                        log::error!("[Server: {index}] Error occured during binding.");
                        log::error!("{error}");
                        std::process::exit(1)
                    })
            }
            None => {
                server = server
                    .bind((server_settings.server_host, server_settings.server_port))
                    .unwrap_or_else(|error| {
                        log::error!("[Server: {index}] Error occured during binding.");
                        log::error!("{error}");
                        std::process::exit(1)
                    })
            }
        }
    }

    server.run().await
}

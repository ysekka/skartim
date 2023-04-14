pub mod application;
pub mod command;

use std::str::FromStr;

use crate::application::r#struct as stcApplication;
use common::commands::article_commands::RemoveArticle;
use common::commands::file_commands::RemoveFile;
use common::commands::file_commands::UploadFile;
use common::commands::user_commands::CreateUser;
use common::commands::user_commands::GetUser;
use common::commands::user_commands::RemoveUser;
use common::commands::user_commands::UpdateUser;
use common::paths::initialization::init_skadmin;
use crate::command::r#enum as enmCommand;
use common::saves::r#struct as stcSaves;
use common::paths::statics::SAVES_PATH;
use common::jwt_claims::Claims;

use common::commands::article_commands::UpdateArticle;
use common::commands::article_commands::CreateArticle;

use env_logger as el;
use serde_json as sj;

use clap::Parser;

pub fn encode_jwt(user_uuid: uuid::Uuid, user_password: &String, daily_password: &String) -> String {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            user_uuid,
            user_password: user_password.clone(),
        },
        &jsonwebtoken::EncodingKey::from_secret(daily_password.as_bytes())
    )
    .unwrap_or_else(|error| {
        log::error!("Error occured during encoding claims. [{error}]");
        std::process::exit(exitcode::SOFTWARE)
    })
}

fn main() {
    el::init_from_env(el::Env::default().default_filter_or("INFO"));

    init_skadmin();

    let application = stcApplication::Application::parse();

    let client = reqwest::blocking::Client::builder()
    .danger_accept_invalid_certs(true)
    .build().unwrap();

    let saves_content = std::fs::read_to_string(SAVES_PATH.as_path())
    .unwrap_or_else(|error| {
        log::error!("Error occured during saves content. [{error}]");
        std::process::exit(exitcode::IOERR)
    });

    let saves = toml::from_str::<stcSaves::Saves>(&saves_content);

    match application.command {
        enmCommand::Command::Server {
            command
        } => {
            match command {
                enmCommand::ServerCommand::Set {
                    value
                } => {
                    match saves {
                        Ok(mut saves) => {
                            saves.server = Some(value);

                            std::fs::write(
                                SAVES_PATH.as_path(),
                                toml::to_string(&saves)
                                .unwrap_or_else(|error| {
                                    log::error!("Error occured during saves`s serialization. [{error}]");
                                    std::process::exit(exitcode::SOFTWARE);
                                })
                            )
                            .unwrap_or_else(|error| {
                                log::error!("Error occured during saving saves. [{error}]");
                                std::process::exit(exitcode::IOERR);
                            });
                        },
                        Err(_) => {
                            std::fs::write(
                                SAVES_PATH.as_path(),
                                toml::to_string(&stcSaves::Saves {
                                    server: Some(value),
                                    login: None
                                })
                                .unwrap_or_else(|error| {
                                    log::error!("Error occured during saves`s serialization. [{error}]");
                                    std::process::exit(exitcode::SOFTWARE);
                                })
                            )
                            .unwrap_or_else(|error| {
                                log::error!("Error occured during saving saves. [{error}]");
                                std::process::exit(exitcode::IOERR);
                            });
                        }
                    }

                    println!("The server variable setted!");
                }
            }
        },
        enmCommand::Command::Login {
            user_uuid,
            user_password,
            daily_password
        } => {
            let saves_content = std::fs::read_to_string(SAVES_PATH.as_path())
            .unwrap_or_else(|error| {
                log::error!("Error occured during saves content. [{error}]");
                std::process::exit(exitcode::IOERR)
            });

            let Ok(stcSaves::Saves {
                server: Some(server),
                login: _
            }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                log::error!("Make sure about that the server variable is setted.");
                std::process::exit(exitcode::NOHOST)
            };

            let query_url = server
            .join("api/private/user/self")
            .unwrap();

            let response = client.get(query_url).bearer_auth(encode_jwt(user_uuid, &user_password, &daily_password))
            .send()
            .unwrap_or_else(|error| {
                log::error!("Error occured during sending request. [{error}]");
                std::process::exit(exitcode::IOERR)
            });

            if response.status().is_success() {
                let mut saves = toml::from_str::<stcSaves::Saves>(&saves_content).unwrap();

                saves.login = Some(stcSaves::LoginInfo {
                    user_uuid,
                    user_password,
                    daily_password
                });

                std::fs::write(
                    SAVES_PATH.as_path(),
                    toml::to_string(&saves)
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during saves`s serialization. [{error}]");
                        std::process::exit(exitcode::SOFTWARE);
                    })
                )
                .unwrap_or_else(|error| {
                    log::error!("Error occured during saving saves. [{error}]");
                    std::process::exit(exitcode::IOERR);
                });

                println!("Logged in!")
            }
            else {
                println!("User not found.");
            }
        }
        enmCommand::Command::Article {
            command
        } => {
            match command {
                enmCommand::ArticleCommand::Create {
                    file
                } => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/article")
                    .unwrap();

                    let article = toml::from_str::<CreateArticle>(
                        &std::fs::read_to_string(file.as_path())
                        .unwrap_or_else(|error| {
                            log::error!("Error occured during reading file. {error}");
                            std::process::exit(exitcode::IOERR)
                        })
                    )
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during deserialization. {error}");
                        std::process::exit(exitcode::SOFTWARE)
                    });

                    let response = client.post(request_url)
                    .query(&article)
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status == http::StatusCode::FORBIDDEN {
                        println!("This action is forbidden.");
                    }
                    else if status.is_server_error() || status.is_client_error() {
                        println!("An error has been occured.")
                    }
                    else if status == http::StatusCode::OK {
                        let article_json = sj::Value::from_str(&response.text().unwrap()).unwrap();
                        let article_uuid = article_json.get("article_uuid").unwrap().as_str().unwrap();

                        println!("The article added to the database with {article_uuid} id.");

                        std::fs::write(file, toml::to_string(&UpdateArticle {
                            article_uuid: uuid::Uuid::from_str(article_uuid).unwrap(),
                            article_title: Some(article.article_title),
                            article_content: Some(article.article_content),
                            article_thumbnail: article.article_thumbnail,
                            article_visibility: article.article_visibility,
                        })
                        .unwrap_or_else(|error| {
                            log::error!("Error occured during serialization. [{error}]");
                            std::process::exit(exitcode::SOFTWARE)
                        }))
                        .unwrap_or_else(|error| {
                            log::error!("Error occured during writing over article file. [{error}]");
                            std::process::exit(exitcode::IOERR)
                        });
                    }
                },
                enmCommand::ArticleCommand::Update {
                    file
                } => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/article")
                    .unwrap();

                    let article = toml::from_str::<CreateArticle>(
                        &std::fs::read_to_string(file.as_path())
                        .unwrap_or_else(|error| {
                            log::error!("Error occured during reading file. {error}");
                            std::process::exit(exitcode::IOERR)
                        })
                    )
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during deserialization. {error}");
                        std::process::exit(exitcode::SOFTWARE)
                    });

                    let response = client.put(request_url)
                    .query(&article)
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status == http::StatusCode::FORBIDDEN {
                        println!("This action is forbidden.");
                    }
                    else if status == http::StatusCode::NOT_FOUND {
                        println!("Article not found");
                    }
                    else if status.is_server_error() || status.is_client_error() {
                        println!("An error has been occured.")
                    }
                    else if status == http::StatusCode::OK {
                        println!("The changes has been saved.")
                    }
                },
                enmCommand::ArticleCommand::Remove {
                    article_uuid
                } => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/article")
                    .unwrap();

                    let response = client.delete(request_url)
                    .query(&RemoveArticle {
                        article_uuid
                    })
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status == http::StatusCode::FORBIDDEN {
                        println!("This action is forbidden.");
                    }
                    else if status == http::StatusCode::NOT_FOUND {
                        println!("Article not found");
                    }
                    else if status.is_server_error() || status.is_client_error() {
                        println!("An error has been occured.")
                    }
                    else if status == http::StatusCode::OK {
                        println!("The article has been deleted.")
                    }
                }
            }
        },
        enmCommand::Command::User {
            command
        } => {
            match command {
                enmCommand::UserCommand::GetSelf => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/user/self")
                    .unwrap();

                    let response = client.get(request_url)
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status.is_success() {
                        let user = sj::Value::from_str(&response.text().unwrap()).unwrap();

                        println!("User ID: [{}]", user.get("user_uuid").unwrap());
                        println!("User Realname: [{}]", user.get("user_realname").unwrap());
                        println!("User Email: [{}]", user.get("user_email").unwrap());
                    }
                    else if status.is_server_error() || status.is_client_error() {
                        println!("An error has been occured.")
                    }
                },
                enmCommand::UserCommand::Get {
                    user_uuid
                } => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/administration/user")
                    .unwrap();

                    let response = client.get(request_url)
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .query(&GetUser {
                        user_uuid
                    })
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status.is_success() {
                        let user = sj::Value::from_str(&response.text().unwrap()).unwrap();

                        println!("User ID: [{}]", user.get("user_uuid").unwrap());
                        println!("User Realname: [{}]", user.get("user_realname").unwrap());
                        println!("User Email: [{}]", user.get("user_email").unwrap());
                    }
                    else if status == http::StatusCode::NOT_FOUND {
                        println!("User not found");
                    }
                    else if status == http::StatusCode::FORBIDDEN {
                        println!("The action is forbidden.")
                    }
                    else if status.is_server_error() || status.is_client_error() {
                        println!("An error has been occured.")
                    }
                },
                enmCommand::UserCommand::Create {
                    user_email,
                    user_status,
                    user_realname,
                    user_password,
                } => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/administration/user")
                    .unwrap();

                    let response = client.post(request_url)
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .query(&CreateUser {
                        user_status,
                        user_email,
                        user_password,
                        user_realname,
                    })
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status.is_success() {
                        let user = sj::Value::from_str(&response.text().unwrap()).unwrap();

                        println!("user has been created with {} id.", user.get("user_uuid").unwrap())
                    }
                    else if status == http::StatusCode::FORBIDDEN {
                        println!("The action is fobidden.");
                    }
                    else if status.is_client_error() || status.is_server_error() {
                        println!("An error has been occured.")
                    }
                },

                enmCommand::UserCommand::Update {
                    user_uuid,
                    user_email,
                    user_password,
                    user_realname,
                } => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/administration/user")
                    .unwrap();

                    let response = client.put(request_url)
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .query(&UpdateUser {
                        user_uuid,
                        user_email,
                        user_password,
                        user_realname,
                    })
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status.is_success() {
                        println!("The changes has been saved.");
                    }
                    else if status == http::StatusCode::FORBIDDEN {
                        println!("The action is fobidden.");
                    }
                    else if status == http::StatusCode::NOT_FOUND {
                        println!("User not found");
                    }
                    else if status.is_client_error() || status.is_server_error() {
                        println!("An error has been occured.")
                    }
                },
                enmCommand::UserCommand::Remove {
                    user_uuid
                } => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/administration/user")
                    .unwrap();

                    let response = client.delete(request_url)
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .query(&RemoveUser {
                        user_uuid,
                    })
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status.is_success() {
                        println!("The user has been removed.");
                    }
                    else if status == http::StatusCode::FORBIDDEN {
                        println!("The action is fobidden.");
                    }
                    else if status == http::StatusCode::NOT_FOUND {
                        println!("User not found");
                    }
                    else if status.is_client_error() || status.is_server_error() {
                        println!("An error has been occured.")
                    }
                }
            }
        },
        enmCommand::Command::File {
            command
        } => {
            match command {
                enmCommand::FileCommand::Upload {
                    file
                } => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/file")
                    .unwrap();

                    let file_name = file.file_name()
                    .unwrap_or_else(|| {
                        log::error!("The path has to have file name.");
                        std::process::exit(exitcode::DATAERR)
                    })
                    .to_str()
                    .unwrap()
                    .to_owned();

                    let response = client.post(request_url)
                    .body(
                        std::fs::read(file.as_path())
                        .unwrap_or_else(|error| {
                            log::error!("Error occured during reading file. [{error}]");
                            std::process::exit(exitcode::IOERR)
                        })
                    )
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .query(&UploadFile {
                        file_name
                    })
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status.is_success() {
                        println!("File has been uploaded.")
                    }
                    else if status.is_server_error() || status.is_client_error() {
                        println!("Error has been occured.")
                    }
                },
                enmCommand::FileCommand::Remove {
                    file_name
                } => {
                    let Ok(stcSaves::Saves {
                        server: Some(server),
                        login: Some(login)
                    }) = toml::from_str::<stcSaves::Saves>(&saves_content) else {
                        log::error!("You have to be logged in.");
                        std::process::exit(exitcode::NOINPUT)
                    };

                    let request_url = server
                    .join("api/private/file")
                    .unwrap();

                    let response = client.delete(request_url)
                    .bearer_auth(encode_jwt(login.user_uuid, &login.user_password, &login.daily_password))
                    .query(&RemoveFile {
                        file_name
                    })
                    .send()
                    .unwrap_or_else(|error| {
                        log::error!("Error occured during sending request. [{error}]");
                        std::process::exit(exitcode::OSERR)
                    });

                    let status = response.status();

                    if status.is_success() {
                        println!("File has been deleted.")
                    }
                    else if status.is_server_error() || status.is_client_error() {
                        println!("Error has been occured.")
                    }
                }
            }
        }
    }
}
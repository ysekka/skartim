use entity::sea_orm_active_enums as soae;

#[derive(clap::Subcommand)]
pub enum Command {
    Server {
        #[command(subcommand)]
        command: ServerCommand
    },

    Login {
        user_uuid: uuid::Uuid,
        user_password: String,
        daily_password: String,
    },

    Article {
        #[command(subcommand)]
        command: ArticleCommand
    },

    User {
        #[command(subcommand)]
        command: UserCommand
    },

    File {
        #[command(subcommand)]
        command: FileCommand
    }
}

#[derive(clap::Subcommand)]
pub enum ServerCommand {
    Set {
        value: url::Url
    }
}

#[derive(clap::Subcommand)]
pub enum ArticleCommand {
    Create {
        file: std::path::PathBuf,
    },

    Update {
        file: std::path::PathBuf,
    },

    Remove {
        article_uuid: uuid::Uuid,
    }
}

#[derive(clap::Subcommand)]
pub enum UserCommand {
    GetSelf,

    Get {
        user_uuid: uuid::Uuid
    },

    Create {
        user_email: String,
        user_password: String,
        user_realname: String,
        user_status: soae::UserStatus,
    },

    Update {
        user_uuid: uuid::Uuid,
        user_email: Option<String>,
        user_password: Option<String>,
        user_realname: Option<String>,
    },

    Remove {
        user_uuid: uuid::Uuid
    },
}

#[derive(clap::Subcommand)]
pub enum FileCommand {
    Upload {
        file: std::path::PathBuf
    },

    Remove {
        file_name: String,
    }
}
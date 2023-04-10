use crate::command::r#enum as enmCommand;

#[derive(clap::Parser)]
#[command(about, author, version = "1.0", long_about = None)]
pub struct Application {
    #[command(subcommand)]
    pub command: enmCommand::Command
}
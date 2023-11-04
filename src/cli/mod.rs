pub mod account;
pub mod error;
pub mod join;

use clap::{Parser, Subcommand};

use self::error::CliError;

type CommandResult = Result<(), CliError>;

#[derive(Debug, Subcommand)]
pub enum Command {
    Account(account::Options),
    Join(join::Options),
}

impl Command {
    pub async fn run(&self) -> CommandResult {
        match self {
            Command::Account(options) => account::run(options).await,
            Command::Join(options) => join::run(options).await,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "rbx-acc", about, version, propagate_version = true)]
pub struct RbxAcc {
    #[command(subcommand)]
    command: Command,
}

impl RbxAcc {
    pub async fn run(&self) -> CommandResult {
        self.command.run().await
    }
}

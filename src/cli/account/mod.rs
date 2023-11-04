mod add;
mod list;
mod login;

use super::CommandResult;
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    Add(add::Options),
    List(list::Options),
    Login(login::Options),
}

impl Command {
    pub async fn run(&self) -> CommandResult {
        match self {
            Command::Add(options) => add::run(options).await,
            Command::List(options) => list::run(options).await,
            Command::Login(options) => login::run(options).await,
        }
    }
}

#[derive(Debug, Args)]
pub struct Options {
    #[command(subcommand)]
    command: Command,
}

pub async fn run(options: &Options) -> CommandResult {
    options.command.run().await
}

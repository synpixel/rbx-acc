mod game;
mod private_server;

use super::CommandResult;
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    Game(game::Options),
    Private(private_server::Options),
}

impl Command {
    pub async fn run(&self) -> CommandResult {
        match self {
            Command::Game(options) => game::run(options).await,
            Command::Private(options) => private_server::run(options).await,
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

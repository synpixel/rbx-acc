use clap::Args;
use reqwest::Client;

use crate::{
    cli::CommandResult,
    utils::users::{format_user, prompt_user_selection},
};

/// Lists every registered account
#[derive(Debug, Args)]
pub struct Options {}

pub async fn run(_options: &Options) -> CommandResult {
    let client = Client::new();
    let user = prompt_user_selection(client).await;

    println!("{}", format_user(&user));

    Ok(())
}

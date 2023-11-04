use clap::Args;
use reqwest::Client;

use crate::{
    api::get_auth_ticket,
    cli::CommandResult,
    utils::{
        launcher::{generate_place_launcher_url, launch_roblox},
        users::prompt_auth_cookie_selection,
    },
};

/// Joins a game instance
#[derive(Debug, Args)]
pub struct Options {
    #[arg(long)]
    place_id: u64,
}

pub async fn run(options: &Options) -> CommandResult {
    let client = Client::new();
    let auth_cookie = prompt_auth_cookie_selection(
        client.to_owned(),
        "Select an account to join the game".to_string(),
    )
    .await;

    let auth_ticket = get_auth_ticket(client, auth_cookie).await.unwrap();
    let place_launcher_url = generate_place_launcher_url(options.place_id);

    launch_roblox(
        auth_ticket.to_str().unwrap().to_string(),
        place_launcher_url,
    );

    Ok(())
}

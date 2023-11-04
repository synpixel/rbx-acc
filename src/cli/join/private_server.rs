use clap::Args;
use reqwest::Client;

use crate::{
    api::get_auth_ticket,
    cli::CommandResult,
    utils::{
        launcher::{
            generate_private_server_launcher_url,
            generate_private_server_launcher_url_with_link_code, launch_roblox,
        },
        private_servers::prompt_private_server_selection,
        users::prompt_auth_cookie_selection,
    },
};

/// Joins a private server
#[derive(Debug, Args)]
pub struct Options {
    #[arg(long)]
    place_id: u64,

    #[arg(long)]
    code: Option<String>,
}

pub async fn run(options: &Options) -> CommandResult {
    let client = Client::new();
    let auth_cookie = prompt_auth_cookie_selection(
        client.to_owned(),
        "Select an account to join the game".to_string(),
    )
    .await;

    let auth_ticket = get_auth_ticket(client.to_owned(), auth_cookie.to_owned())
        .await
        .unwrap();
    let place_launcher_url = if options.code.is_some() {
        generate_private_server_launcher_url_with_link_code(
            options.place_id,
            options.code.to_owned().unwrap(),
        )
    } else {
        let private_server = prompt_private_server_selection(
            client,
            auth_cookie,
            options.place_id,
            "Select a private server to join".to_string(),
        )
        .await
        .unwrap();

        generate_private_server_launcher_url(options.place_id, private_server.access_code)
    };

    launch_roblox(
        auth_ticket.to_str().unwrap().to_string(),
        place_launcher_url,
    );

    Ok(())
}

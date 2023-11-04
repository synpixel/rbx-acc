use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use reqwest::Client;

use crate::api::{get_private_servers, PrivateServer};

pub async fn prompt_private_server_selection(
    client: Client,
    auth_cookie: String,
    place_id: u64,
    prompt: String,
) -> Result<PrivateServer, Box<dyn std::error::Error>> {
    let mut next_page_cursor: Option<String> = None;
    let mut private_servers: Vec<PrivateServer> = vec![];

    loop {
        let private_servers_response = get_private_servers(
            client.to_owned(),
            auth_cookie.to_owned(),
            place_id,
            100,
            next_page_cursor,
            "Asc".to_string(),
        )
        .await
        .unwrap();

        for private_server in private_servers_response.data {
            private_servers.push(private_server);
        }

        next_page_cursor = private_servers_response.next_page_cursor;

        if next_page_cursor.is_none() {
            break;
        }
    }

    let formatted_private_servers: Vec<_> = private_servers
        .iter()
        .map(|private_server| {
            format!(
                "{} {}",
                private_server.name.as_str().bold(),
                format!("@{}", private_server.owner.name).as_str().black()
            )
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&formatted_private_servers)
        .interact()
        .unwrap();

    Ok(private_servers.get(selection).unwrap().clone())
}

use std::{thread, time::Duration};

use clap::Args;
use headless_chrome::{protocol::cdp::Network::CookieParam, Browser, LaunchOptionsBuilder};
use reqwest::Client;

use crate::{cli::CommandResult, utils::users::prompt_auth_cookie_selection};

/// Logs into a registered account
#[derive(Debug, Args)]
pub struct Options {}

pub async fn run(_options: &Options) -> CommandResult {
    let client = Client::new();
    let auth_cookie =
        prompt_auth_cookie_selection(client, "Select an account to log into".to_string()).await;

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(false)
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.new_tab().unwrap();

    tab.set_cookies(vec![CookieParam {
        name: ".ROBLOSECURITY".to_string(),
        value: auth_cookie,
        url: Some("https://roblox.com/".to_string()),
        domain: Some(".roblox.com".to_string()),
        path: Some("/".to_string()),
        http_only: Some(true),
        secure: Some(true),
        expires: None,
        same_site: None,
        priority: None,
        same_party: None,
        source_scheme: None,
        source_port: None,
        partition_key: None,
    }])
    .unwrap();

    tab.navigate_to("https://roblox.com/home").unwrap();

    thread::sleep(Duration::from_secs(60));

    Ok(())
}

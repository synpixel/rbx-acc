use clap::Args;
use reqwest::Client;
use std::time::Duration;

use crate::{api::get_authenticated_user, auth::get_auth_cookie};
use crate::{
    cli::{error::CliError, CommandResult},
    save::add_account,
};
use crate::{save::get_save, utils::users::format_user_minimal};

/// Add an account to the registry
#[derive(Debug, Args)]
pub struct Options {}

pub async fn run(_options: &Options) -> CommandResult {
    let client = Client::new();
    let save = get_save();

    let auth_cookie = get_auth_cookie(Duration::from_secs(1)).unwrap();

    if save.accounts.contains(&auth_cookie) {
        println!("Account already registered.");
        return Err(CliError::new(1));
    }

    add_account(auth_cookie.clone());

    let authenticated_user = get_authenticated_user(client, auth_cookie.clone())
        .await
        .unwrap();

    println!("Registered {}.", format_user_minimal(&authenticated_user));

    Ok(())
}

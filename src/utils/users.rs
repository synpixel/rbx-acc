use std::borrow::Cow;

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use reqwest::Client;

use crate::{
    api::{get_authenticated_user, AuthenticatedUserResponse},
    save::get_save,
};

pub fn format_user_minimal(user: &AuthenticatedUserResponse) -> Cow<str> {
    if user.name == user.display_name {
        return user.name.as_str().into();
    }

    format!(
        "{} {}",
        user.display_name,
        format!("(@{})", user.name).as_str().black()
    )
    .into()
}

pub fn format_user(user: &AuthenticatedUserResponse) -> String {
    format!(
        "{}\n\n{}\n{}\n\n{}",
        "-".repeat(40).cyan(),
        user.display_name,
        format!("@{}", user.name).as_str().black(),
        "-".repeat(40).cyan()
    )
}

pub async fn prompt_user_selection(client: Client) -> AuthenticatedUserResponse {
    let save = get_save();
    let mut authenticated_users: Vec<AuthenticatedUserResponse> = vec![];

    for auth_cookie in save.accounts.iter() {
        let authenticated_user = get_authenticated_user(client.clone(), auth_cookie.clone())
            .await
            .unwrap();
        authenticated_users.push(authenticated_user);
    }

    let formatted_users: Vec<_> = authenticated_users
        .iter()
        .map(format_user_minimal)
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&formatted_users)
        .interact()
        .unwrap();

    authenticated_users.get(selection).unwrap().clone()
}

pub async fn prompt_auth_cookie_selection(client: Client, prompt: String) -> String {
    let save = get_save();
    let mut authenticated_users: Vec<AuthenticatedUserResponse> = vec![];

    for auth_cookie in save.accounts.iter() {
        let authenticated_user = get_authenticated_user(client.clone(), auth_cookie.clone())
            .await
            .unwrap();
        authenticated_users.push(authenticated_user);
    }

    let formatted_users: Vec<_> = authenticated_users
        .iter()
        .map(format_user_minimal)
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&formatted_users)
        .interact()
        .unwrap();

    save.accounts.get(selection).unwrap().into()
}

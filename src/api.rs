use reqwest::{
    header::{self, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};

use crate::auth::get_csrf_token;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUserResponse {
    pub id: u64,
    pub name: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPresencesResponse {
    pub user_presences: Vec<UserPresence>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServersResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<PrivateServer>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPresenceRequest {
    pub user_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
    pub user_presence_type: i32,
    pub last_location: String,
    pub place_id: Option<u64>,
    pub root_place_id: Option<u64>,
    pub game_id: Option<String>,
    pub universe_id: Option<u64>,
    pub user_id: u64,
    pub last_online: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PrivateServer {
    pub name: String,
    pub vip_server_id: u64,
    pub access_code: String,
    pub owner: VerifiedBadgeUserResponse,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedBadgeUserResponse {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub has_verified_badge: bool,
}

pub async fn get_authenticated_user(
    client: Client,
    auth_cookie: String,
) -> Result<AuthenticatedUserResponse, reqwest::Error> {
    client
        .get("https://users.roblox.com/v1/users/authenticated")
        .header(header::COOKIE, format!(".ROBLOSECURITY={}", auth_cookie))
        .send()
        .await?
        .json::<AuthenticatedUserResponse>()
        .await
}

pub async fn get_auth_ticket(
    client: Client,
    auth_cookie: String,
) -> Result<HeaderValue, reqwest::Error> {
    let csrf_token = get_csrf_token(client.to_owned(), auth_cookie.to_owned()).await?;
    let csrf_token_str = csrf_token.to_str().unwrap();

    Ok(client
        .post("https://auth.roblox.com/v1/authentication-ticket")
        .header(header::COOKIE, format!(".ROBLOSECURITY={}", auth_cookie))
        .header("X-Csrf-Token", csrf_token_str)
        .header("Referer", "https://www.roblox.com")
        .send()
        .await?
        .headers()
        .get("rbx-authentication-ticket")
        .unwrap()
        .to_owned())
}

/*
pub async fn get_user_presences(
    client: Client,
    auth_cookie: String,
    user_ids: Vec<u64>,
) -> Result<UserPresencesResponse, reqwest::Error> {
    client
        .get("https://presence.roblox.com/v1/presence/users")
        .header(header::COOKIE, format!(".ROBLOSECURITY={}", auth_cookie))
        .body(serde_json::to_string(&UserPresenceRequest { user_ids }).unwrap())
        .send()
        .await?
        .json::<UserPresencesResponse>()
        .await
}
*/

pub async fn get_private_servers(
    client: Client,
    auth_cookie: String,
    place_id: u64,
    limit: u32,
    cursor: Option<String>,
    sort_order: String,
) -> Result<PrivateServersResponse, reqwest::Error> {
    client
        .get(format!(
            "https://games.roblox.com/v1/games/{}/private-servers?limit={}&cursor={}&sortOrder={}
    ",
            place_id,
            limit,
            cursor.unwrap_or("".to_string()),
            sort_order
        ))
        .header(header::COOKIE, format!(".ROBLOSECURITY={}", auth_cookie))
        .send()
        .await?
        .json::<PrivateServersResponse>()
        .await
}

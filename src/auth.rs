use std::{error::Error, thread, time::Duration};

use headless_chrome::{Browser, LaunchOptionsBuilder};
use reqwest::{
    header::{self, HeaderValue},
    Client,
};

pub fn get_auth_cookie(poll_interval: Duration) -> Result<String, Box<dyn Error>> {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(false)
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://roblox.com/login").unwrap();
    tab.wait_for_element("#login-button").unwrap();

    let auth_cookie: String;

    'auth_cookie_yielder: loop {
        for cookie in tab.get_cookies().unwrap() {
            if cookie.name == ".ROBLOSECURITY" {
                auth_cookie = cookie.value;
                break 'auth_cookie_yielder;
            }
        }
        thread::sleep(poll_interval);
    }

    Ok(auth_cookie)
}

pub async fn get_csrf_token(
    client: Client,
    auth_cookie: String,
) -> Result<HeaderValue, reqwest::Error> {
    Ok(client
        .post("https://auth.roblox.com/v1/authentication-ticket")
        .header(header::COOKIE, format!(".ROBLOSECURITY={}", auth_cookie))
        .send()
        .await?
        .headers()
        .get("x-csrf-token")
        .unwrap()
        .to_owned())
}

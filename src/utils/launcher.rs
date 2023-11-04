use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn launch_roblox(auth_ticket: String, place_launcher_url: String) {
    let progress = ProgressBar::new_spinner()
        .with_style(
            ProgressStyle::with_template("{spinner:.cyan} {msg}")
                .unwrap()
                .tick_chars("⠁⠈⠐⠠⠄⠂"),
        )
        .with_message("Launching Roblox Player...");

    progress.enable_steady_tick(Duration::from_millis(100));

    let protocol_url = format!(
        "roblox-player:1+launchmode:play+gameinfo:{}+placelauncherurl:{}",
        auth_ticket, place_launcher_url
    );

    open::that(protocol_url).unwrap();
    progress.finish_and_clear();
}

pub fn generate_place_launcher_url(place_id: u64) -> String {
    urlencoding::encode(&format!(
        "https://assetgame.roblox.com/game/PlaceLauncher.ashx?request=RequestGame&placeId={}",
        place_id
    ))
    .to_string()
}

pub fn generate_private_server_launcher_url(place_id: u64, access_code: String) -> String {
    urlencoding::encode(&format!(
        "https://assetgame.roblox.com/game/PlaceLauncher.ashx?request=RequestPrivateGame&placeId={}&accessCode={}",
        place_id,
        access_code
    ))
    .to_string()
}

pub fn generate_private_server_launcher_url_with_link_code(
    place_id: u64,
    link_code: String,
) -> String {
    urlencoding::encode(&format!(
        "https://assetgame.roblox.com/game/PlaceLauncher.ashx?request=RequestPrivateGame&placeId={}&accessCode=&linkCode={}",
        place_id,
        link_code
    ))
    .to_string()
}

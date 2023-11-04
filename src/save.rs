use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Save {
    pub accounts: Vec<String>,
}

fn get_save_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap();
    let home_dir_str = home_dir.to_str().unwrap();
    let save_path_string = format!("{}/.rbx-acc", home_dir_str);
    let mut save_path = PathBuf::new();
    save_path.push(save_path_string);
    save_path
}

pub fn get_save() -> Save {
    let save_path = get_save_path();
    if let Ok(save) = file::get_text(save_path) {
        serde_json::from_str(save.as_str()).unwrap()
    } else {
        set_save(Save::default());
        get_save()
    }
}

pub fn set_save(save: Save) {
    let save_path = get_save_path();
    let save = serde_json::to_string::<Save>(&save);
    file::put_text(save_path, save.unwrap()).unwrap();
}

pub fn add_account(auth_cookie: String) {
    let mut save = get_save();
    save.accounts.push(auth_cookie);
    set_save(save);
}

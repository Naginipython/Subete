use crate::{save, FILE_PATH};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, path::PathBuf, sync::LazyLock};

use super::EpisodeItem;

static UPDATE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = (*FILE_PATH).clone();
    path.push("anime_updates.json");
    path
});

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateItem {
    pub id: String,
    pub title: String,
    pub img: String,
    pub plugin: String,
    pub episode: EpisodeItem,
}

#[tauri::command]
pub fn save_anime_updates_list(items: Vec<UpdateItem>) {
    println!("Saving anime updates list...");
    save(&*UPDATE_PATH, &items);
}
#[tauri::command]
pub fn get_anime_updates_list() -> Value {
    // todo: fix unwraps
    println!("Getting anime updates list...");
    let updates: Vec<UpdateItem> = match File::open(&*UPDATE_PATH) {
        Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
        Err(_e) => {
            File::create(&*UPDATE_PATH).unwrap();
            Vec::new()
        }
    };
    serde_json::to_value(updates).unwrap()
}

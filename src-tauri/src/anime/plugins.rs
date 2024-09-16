use std::{fs::File, path::PathBuf, sync::{LazyLock, Mutex}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::{get_item, get_list, save, search, IsPlugin, Media, FILE_PATH};

use super::LibraryItem;

static PLUGINS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = (*FILE_PATH).clone();
    path.push("anime_plugins.json");
    path
});
static PLUGINS: LazyLock<Mutex<Vec<Plugins>>> = LazyLock::new(|| match File::open(&*PLUGINS_PATH) {
    Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
    Err(_e) => {
        File::create(&*PLUGINS_PATH).unwrap();
        Mutex::new(Vec::new())
    }
});

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Plugins {
    pub id: String,
    pub media_type: Media,
    pub search_url: String,
    pub search: String,
    pub search_extra: Value,
    pub episodes_url: String,
    pub get_episodes: String,
    pub episodes_extra: Value,
    pub videos_url: String,
    pub get_videos: String,
    pub videos_extra: Value,
}
impl IsPlugin for Plugins {
    fn id(&self) -> String { self.id.clone() }
    fn search_url(&self) -> &str { &self.search_url }
    fn search(&self) -> &str { &self.search }
    fn list_url(&self) -> &str { &self.episodes_url }
    fn get_list(&self) -> &str { &self.get_episodes }
    fn list_extra(&self) -> &Value { &self.episodes_extra }
    fn item_url(&self) -> &str { &self.videos_url }
    fn get_item(&self) -> &str { &self.get_videos }
    fn item_fn(&self) -> &str { "getEpisodeVideo" }
}

fn get_plugins() -> Vec<Plugins> {
    match File::open(&*PLUGINS_PATH) {
        Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
        Err(_e) => {
            let plugins = vec![];
            save(&*PLUGINS_PATH, &plugins);
            plugins
        }
    }
}
fn init_plugins() -> Vec<Plugins> {
    File::create(&*PLUGINS_PATH).unwrap();
    let plugins = vec![];
    save(&*PLUGINS_PATH, &plugins);
    plugins
}

#[tauri::command]
pub fn add_anime_plugin(new_plugin: Plugins) {
    println!("Adding anime plugin...");
    let mut plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    if !names.contains(&new_plugin.id) {
        plugins.push(new_plugin);
        save(&*PLUGINS_PATH, &plugins);
    }
}

#[tauri::command]
pub fn get_anime_plugin_names() -> Value {
    println!("Getting Anime Plugin Names...");
    let plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    json!(names)
}

#[tauri::command]
pub async fn anime_search(query: String, sources: Vec<String>) -> Value {
    search("anime", get_plugins(), query, sources).await
}

#[tauri::command]
pub async fn get_anime_episodes(anime: LibraryItem) -> Value {
    get_list("anime", get_plugins(), anime).await
}

#[tauri::command]
pub async fn get_anime_video(source: String, id: String) -> Value {
    get_item("anime", get_plugins(), source, id).await
}

#[tauri::command]
pub fn delete_anime_plugin(plugin: String) {
    println!("Deleting anime plugin: {plugin}");
    let mut plugins = PLUGINS.lock().unwrap();
    plugins.retain(|p| p.id != plugin);
    save(&*PLUGINS_PATH, &plugins);
}

#[tauri::command]
pub fn delete_anime_plugins() {
    println!("Deleting anime plugins...");
    let mut plugins = PLUGINS.lock().unwrap();
    *plugins = init_plugins();
    std::fs::remove_file(&*PLUGINS_PATH).unwrap();
}

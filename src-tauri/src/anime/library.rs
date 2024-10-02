use crate::{
    add_to_lib, delete_entire_lib, get_lib, remove_from_lib, update_lib, HasId, IsItem, FILE_PATH,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::File,
    path::PathBuf,
    process::Command,
    sync::{LazyLock, Mutex},
};

static LIB_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = (*FILE_PATH).clone();
    path.push("anime_library.json");
    path
});
static LIB: LazyLock<Mutex<Vec<LibraryItem>>> = LazyLock::new(|| match File::open(&*LIB_PATH) {
    Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
    Err(_e) => {
        File::create(&*LIB_PATH).unwrap();
        Mutex::new(Vec::new())
    }
});

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LibraryItem {
    pub id: String,
    pub title: String,
    pub img: String,
    pub plugin: String,
    pub studio: String,
    pub status: String,
    pub description: Option<String>,
    pub episodes: Vec<EpisodeItem>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EpisodeItem {
    pub id: String,
    pub number: f64,
    pub title: String,
    pub watch_time: f32,
    pub duration: f32,
    pub completed: bool,
}
impl PartialEq for LibraryItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.plugin == other.plugin
    }
}
impl HasId for LibraryItem {
    fn id(&self) -> &str {
        &self.id
    }
}
impl IsItem for LibraryItem {
    fn plugin(&self) -> &str {
        &self.plugin
    }
    fn list_fn(&self) -> &str {
        "getEpisodes"
    }
}

#[tauri::command]
pub fn get_anime_lib() -> Value {
    get_lib("anime", &LIB)
}

#[tauri::command]
pub fn add_to_anime_lib(new_item: LibraryItem) {
    add_to_lib("anime", &LIB, &LIB_PATH, new_item)
}

#[tauri::command]
pub fn update_anime_lib(item: LibraryItem) {
    update_lib("anime", &LIB, &LIB_PATH, item)
}

#[tauri::command]
pub fn remove_from_anime_lib(id: String) {
    remove_from_lib("anime", &LIB, &LIB_PATH, id)
}

#[tauri::command]
pub fn delete_anime_lib() {
    delete_entire_lib("anime", &LIB, &LIB_PATH);
}

#[tauri::command]
pub async fn open_in_vlc(url: String) {
    let mut process = Command::new("vlc").arg(url).spawn().expect("Failed");
    process.wait().expect("Failed2");
}

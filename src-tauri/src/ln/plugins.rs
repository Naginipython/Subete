use std::{fs::File, io::Write, path::PathBuf, sync::{LazyLock, Mutex}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri_plugin_http::reqwest;
use crate::{get_item, get_list, save, search, IsPlugin, Media, DOWNLOADS_PATH, FILE_PATH};

use super::LibraryItem;

static PLUGINS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = (*FILE_PATH).clone();
    path.push("ln_plugins.json");
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
    pub chapters_url: String,
    pub get_chapters: String,
    pub chapters_extra: Value,
    pub pages_url: String,
    pub get_pages: String,
    pub pages_extra: Value,
}
impl IsPlugin for Plugins {
  fn id(&self) -> String { self.id.clone() }
  fn search_url(&self) -> &str { &self.search_url }
  fn search(&self) -> &str { &self.search }
  fn list_url(&self) -> &str { &self.chapters_url }
  fn get_list(&self) -> &str { &self.get_chapters }
  fn list_extra(&self) -> &Value { &self.chapters_extra }
  fn item_url(&self) -> &str { &self.pages_url }
  fn get_item(&self) -> &str { &self.get_pages }
  fn item_fn(&self) -> &str { "getChapterPages" }
}

fn get_plugins() -> Vec<Plugins> {
    match File::open(&*PLUGINS_PATH) {
        Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
        Err(_e) => init_plugins(),
    }
}
fn init_plugins() -> Vec<Plugins> {
    File::create(&*PLUGINS_PATH).unwrap();
    let plugins = vec![];
    save(&*PLUGINS_PATH, &plugins);
    plugins
}

#[tauri::command]
pub fn add_ln_plugin(new_plugin: Plugins) {
    println!("Adding ln plugin...");
    let mut plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    if !names.contains(&new_plugin.id) {
        plugins.push(new_plugin);
        save(&*PLUGINS_PATH, &plugins);
    }
}

#[tauri::command]
pub fn get_ln_plugin_names() -> Value {
    println!("Getting Ln Plugin Names...");
    let plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    json!(names)
}

#[tauri::command]
pub async fn ln_search(query: String, source: String) -> Value {
    search("ln", get_plugins(), query, source).await
}

#[tauri::command]
pub async fn get_ln_chapters(item: LibraryItem) -> Value {
    get_list("ln", get_plugins(), item).await
}

#[tauri::command]
pub async fn get_ln_pages(source: String, id: String) -> Value {
    get_item("ln", get_plugins(), source, id).await
}

#[tauri::command]
pub fn delete_ln_plugin(plugin: String) {
    println!("Deleting ln plugin: {plugin}");
    let mut plugins = PLUGINS.lock().unwrap();
    plugins.retain(|p| p.id != plugin);
    save(&*PLUGINS_PATH, &plugins);
}

#[tauri::command]
pub fn delete_ln_plugins() {
    println!("Deleting ln plugins...");
    let mut plugins = PLUGINS.lock().unwrap();
    *plugins = init_plugins();
    std::fs::remove_file(&*PLUGINS_PATH).unwrap();
}

#[tauri::command]
pub async fn download_ln(url: String) {
    println!("Downloading ln...");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();
    let mut file_path = DOWNLOADS_PATH.clone();
    let data = response.bytes().await.unwrap();
    file_path.push("test.pdf");
    let mut file = File::create(file_path).unwrap();
    file.write_all(&data).unwrap();
    println!("done");
}
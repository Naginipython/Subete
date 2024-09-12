use crate::{fetch, post_fetch, save, search, IsPlugin, Media, FILE_PATH};
use std::{fs::File, path::PathBuf, sync::{LazyLock, Mutex}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    fn id(&self) -> String {
        self.id.clone()
    }
    fn search_url(&self) -> &str {
        &self.search_url
    }
    fn search(&self) -> &str {
        &self.search
    }
}

fn replace_url(url: &str, placeholder: &str, replace: &str) -> String {
  url.replace(placeholder, replace)
}
fn get_plugins() -> Vec<Plugins> {
  match File::open(&*PLUGINS_PATH) {
      Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
      Err(_e) => init_plugins()
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
pub async fn ln_search(query: String, sources: Vec<String>) -> Value {
  search("ln", get_plugins(), query, sources).await
}

#[tauri::command]
pub async fn get_ln_chapters(ln: LibraryItem) -> Value {
    println!("Getting ln chapters...");
    let result: Value = json!({});
    let plugins = get_plugins();
    println!("{:?}", plugins);
    let plugin = plugins.iter().find(|p| p.id == ln.plugin);
    if let Some(p) = plugin {
        // let temp = json!({"url": replace_url(&p.chapters_url, "{id}", &id), "getChapters": (p.get_chapters).to_string(), "extra": p.chapters_extra});
        // result = temp;
        let url = replace_url(&p.chapters_url, "{id}", &ln.id);
        let html = if p.chapters_extra.get("request").is_some() {
            post_fetch(url).await
        } else {
            fetch(url).await
        };
        
        let mut chap_code = (&p.get_chapters).clone();
        chap_code.push_str(&format!("getChapters(JSON.parse({:?}), `{html}`);", serde_json::to_string(&ln).unwrap()));
        
        // let context = quickjs_rs::Context::new().unwrap();
        // let value = context.eval(&chap_code).unwrap();
        // result = js_value_to_serde_json(value);
    }
    result
}

#[tauri::command]
pub async fn get_ln_pages(source: String, id: String) -> Value {
  println!("Getting ln pages...");
    let result: Value = json!({});
    let plugins = get_plugins();
    let plugin = plugins.iter().find(|p| p.id == source);
    if let Some(p) = plugin {
        // let temp = json!({"url": replace_url(&p.pages_url, "{id}", &id), "getChapterPages": (p.get_pages).to_string()});
        // result = temp;
        let url = replace_url(&p.pages_url, "{id}", &id);
        let html = fetch(url).await;
        
        let mut chap_code = (&p.get_pages).clone();
        chap_code.push_str(&format!("getChapterPages(`{html}`);"));
        
        // let context = quickjs_rs::Context::new().unwrap();
        // let value = context.eval(&chap_code).unwrap();
        // result = js_value_to_serde_json(value);
    }
    result
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
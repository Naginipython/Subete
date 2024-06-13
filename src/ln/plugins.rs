use crate::FILE_PATH;
use std::{fs::File, io::Write, sync::Mutex};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use lazy_static::lazy_static;


lazy_static! {
  pub static ref PLUGINS_PATH: String = {
    let mut path = (*FILE_PATH).clone();
    path.push_str("/ln_plugins.json");
    path
  };
  static ref PLUGINS: Mutex<Vec<Plugins>> = match File::open(&*PLUGINS_PATH) {
    Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
    Err(_e) => {
      let plugins = init_plugins();
      Mutex::new(plugins)
    }
  };
}

fn save(lib: &Vec<Plugins>) {
  let mut file = File::create(&*PLUGINS_PATH).unwrap();
  let json = serde_json::to_string(&*lib).unwrap();
  file.write_all(json.as_bytes()).unwrap();
}
fn get_plugins() -> Vec<Plugins> {
  match File::open(&*PLUGINS_PATH) {
      Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
      Err(_e) => init_plugins()
  }
}
#[tauri::command]
pub fn add_ln_plugin(new_plugin: Plugins) {
  println!("Adding ln plugin...");
  let mut plugins = PLUGINS.lock().unwrap();
  let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
  if !names.contains(&new_plugin.id) {
    plugins.push(new_plugin);
    save(&*plugins);
  }
}
fn init_plugins() -> Vec<Plugins> {
  File::create(&*PLUGINS_PATH).unwrap();
  let plugins = vec![];
  save(&plugins);
  plugins
}

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
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Media {
    #[serde(rename = "manga")]
    Manga, 
    #[serde(rename = "ln")]
    Ln, 
    #[serde(rename = "anime")]
    Anime
}
impl Default for Media {
    fn default() -> Self { Media::Manga }
}

#[tauri::command]
pub fn get_ln_plugin_names() -> Value {
  println!("Getting Ln Plugin Names...");
  let plugins = PLUGINS.lock().unwrap();
  let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
  json!(names)
}

fn replace_url(url: &String, placeholder: &str, replace: &str) -> String {
  url.replace(placeholder, replace)
}

#[tauri::command]
pub async fn ln_search(query: String, sources: Vec<String>) -> Value {
  println!("Searching Ln(s)...");
  let mut result: Vec<Value> = Vec::new();
  let plugins = get_plugins();
  for p in plugins {
      if sources.contains(&p.id) {
          let temp = json!({"url": replace_url(&p.search_url, "{title}", &query), "search": (p.search).to_string(), "extra": p.search_extra});
          result.push(temp);
      }
  }
  json!(result)
}

#[tauri::command]
pub async fn get_ln_chapters(source: String, id: String) -> Value {
    println!("Getting ln chapters...");
    let mut result: Value = json!({});
    let plugins = get_plugins();
    let plugin = plugins.iter().find(|p| p.id == source);
    if let Some(p) = plugin {
        let temp = json!({"url": replace_url(&p.chapters_url, "{id}", &id), "getChapters": (p.get_chapters).to_string(), "extra": p.chapters_extra});
        result = temp;
    }
    result
}

#[tauri::command]
pub async fn get_ln_pages(source: String, id: String) -> Value {
  println!("Getting ln pages...");
    let mut result: Value = json!({});
    let plugins = get_plugins();
    let plugin = plugins.iter().find(|p| p.id == source);
    if let Some(p) = plugin {
        let temp = json!({"url": replace_url(&p.pages_url, "{id}", &id), "getChapterPages": (p.get_pages).to_string(), "extra": p.chapters_extra});
        result = temp;
    }
    result
}
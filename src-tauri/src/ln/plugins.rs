use crate::{fetch, post_fetch, Media, FILE_PATH};
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf, sync::{LazyLock, Mutex}};
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

fn save(p: &Vec<Plugins>) {
  let mut file = File::create(&*PLUGINS_PATH).unwrap();
  let json = serde_json::to_string(p).unwrap();
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
    save(&plugins);
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

#[tauri::command]
pub fn get_ln_plugin_names() -> Value {
  println!("Getting Ln Plugin Names...");
  let plugins = PLUGINS.lock().unwrap();
  let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
  json!(names)
}

fn replace_url(url: &str, placeholder: &str, replace: &str) -> String {
  url.replace(placeholder, replace)
}

#[tauri::command]
pub async fn ln_search(query: String, sources: Vec<String>) -> Value {
  println!("Searching Ln(s)...");
  let mut result: Value = json!({});
  let plugins = get_plugins();
  for p in plugins {
      if sources.contains(&p.id) {
          // Fetching page data
          let url = replace_url(&p.search_url, "{title}", &query);
          let html = fetch(url).await;

          // Getting from plugins
          let search = (p.search).to_string();
          let search1 = format!("{}search(`{}`);", &search, &html);
          
          // let context = quickjs_rs::Context::new().unwrap();
          // let value = context.eval(&search1).unwrap_or_else(|_e| {
          //     // secondary test
          //     let search2 = format!("{}search(JSON.stringify({}));", &search, &html);
          //     match context.eval(&search2) {
          //       Ok(v) => v,
          //       Err(e) => {
          //         let h = HashMap::from([(String::from("error"), JsValue::String(format!("{:?} experienced an issue: {e}", p.id)))]);
          //         JsValue::Object(h)
          //       }
          //     }
          // });
          // let r = js_value_to_serde_json(value);
          // append_values(&mut result, r)
      }
  }
  json!(result)
}

#[tauri::command]
pub async fn get_ln_chapters(ln: LibraryItem) -> Value {
    println!("Getting ln chapters...");
    let mut result: Value = json!({});
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
    let mut result: Value = json!({});
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
    save(&plugins);
}

#[tauri::command]
pub fn delete_ln_plugins() {
    println!("Deleting ln plugins...");
    let mut plugins = PLUGINS.lock().unwrap();
    *plugins = init_plugins();
    std::fs::remove_file(&*PLUGINS_PATH).unwrap();
}
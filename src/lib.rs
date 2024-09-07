use std::{fs::File, io::Write, path::PathBuf};
use lazy_static::lazy_static;
use serde::Serialize;
use settings::*;
use tauri_plugin_http::reqwest;
use quickjs_rs::JsValue;
use serde_json::{Value, Map as JsonMap, Number as JsonNumber};

mod manga;
mod ln;
mod settings;

lazy_static! {
    pub static ref FILE_PATH: PathBuf = {
      let mut data_folder: PathBuf = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("./"));
      // let mut str = data_folder.to_str().unwrap_or_default().to_string();
      let os = std::env::consts::OS;
      // if os == "windows" {
      //   data_folder.join("\\subete");
      if os == "android" {
        data_folder = PathBuf::from("/data/data/com.subete.dev/files/");
      } else {
        data_folder.push("subete");
      }
      std::fs::create_dir_all(&data_folder).unwrap_or_default();
      data_folder
    };
  }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .setup(|_app| {
      // let window = app.get_webview_window("main").unwrap();
      // window.set_fullscreen(true);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        fetch, post_fetch,
        // manga/library.rs
        manga::get_manga_lib, manga::add_to_manga_lib, manga::remove_from_manga_lib, manga::update_manga_lib, manga::delete_manga_lib,
        // manga/plugins.rs
        manga::manga_search, manga::get_manga_chapters, manga::get_manga_plugin_names, manga::get_manga_pages, manga::add_manga_plugin, 
        manga::delete_manga_plugins, manga::delete_manga_plugin,
        // manga/update.rs
        manga::save_manga_updates_list, manga::get_manga_updates_list,
        // manga/history.rs
        manga::save_manga_history, manga::get_manga_history,
        // ln/library.rs
        ln::get_ln_lib, ln::add_to_ln_lib, ln::remove_from_ln_lib, ln::update_ln_lib, ln::delete_ln_lib,
        // ln/plugins.rs
        ln::ln_search, ln::get_ln_chapters, ln::get_ln_plugin_names, ln::get_ln_pages, ln::add_ln_plugin, ln::delete_ln_plugins, 
        ln::delete_ln_plugin,
        // settings.rs
        update_settings, get_settings, delete_settings
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn save<U: Serialize>(path: &PathBuf, data: &Vec<U>) {
  let mut file = File::create(path).unwrap();
  let json = serde_json::to_string(data).unwrap();
  file.write_all(json.as_bytes()).unwrap();
}

#[tauri::command]
async fn fetch(url: String) -> String {
  let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
  let client = reqwest::Client::new();
  let response = client.get(url)
      .header(reqwest::header::USER_AGENT, user_agent)
      .send()
      .await.unwrap();
  let mut data = response.text().await.unwrap();
  data = data.replace("\n", " ").replace('`', "").replace("${", "S").replace("\\\"", "'");
  let re = regex::Regex::new(r"\s+").unwrap();
  data = re.replace_all(&data, " ").to_string();
  data
}

#[tauri::command]
async fn post_fetch(url: String) -> String {
  let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
  let client = reqwest::Client::new();
  let response = client.post(url)
    .header(reqwest::header::USER_AGENT, user_agent)
    .send()
    .await.unwrap();
  let mut data = response.text().await.unwrap();
  data = data.replace("\n", " ").replace('`', "").replace("${", "S").replace("\\\"", "'");
  let re = regex::Regex::new(r"\s+").unwrap();
  data = re.replace_all(&data, " ").to_string();
  data
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub enum Media {
    #[serde(rename = "manga")]
    #[default]
    Manga, 
    #[serde(rename = "ln")]
    Ln, 
    #[serde(rename = "anime")]
    Anime
}

pub fn js_value_to_serde_json(value: JsValue) -> Value {
  match value {
      JsValue::Null => Value::Null,
      JsValue::Bool(b) => Value::Bool(b),
      JsValue::Int(i) => Value::Number(JsonNumber::from(i)),
      JsValue::Float(f) => Value::Number(JsonNumber::from_f64(f).unwrap()),
      JsValue::String(s) => Value::String(s),
      JsValue::Array(arr) => {
          let json_array: Vec<Value> = arr.into_iter().map(js_value_to_serde_json).collect();
          Value::Array(json_array)
      },
      JsValue::Object(obj) => {
          let json_map: JsonMap<String, Value> = obj.into_iter()
              .map(|(k, v)| (k, js_value_to_serde_json(v)))
              .collect();
            Value::Object(json_map)
      },
      _ => unimplemented!("Unsupported JsValue type for conversion"),
  }
}
pub fn append_values(v1: &mut Value, v2: Value) {
  if let (Value::Array(ref mut arr1), Value::Array(arr2)) = (v1, v2) {
      arr1.extend(arr2);
  }
}
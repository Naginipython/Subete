use std::path::PathBuf;
use lazy_static::lazy_static;
use settings::*;
use tauri_plugin_http::reqwest;

mod manga;
mod ln;
mod settings;

lazy_static! {
    pub static ref FILE_PATH: String = {
      let data_folder: PathBuf = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("./"));
      let mut str = data_folder.to_str().unwrap_or_default().to_string();
      str.push_str("/omniyomi");
      str
    };
  }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            fetch, post_fetch,
            // manga/library.rs
            manga::get_manga_lib, manga::add_to_manga_lib, manga::remove_from_manga_lib, manga::update_manga_lib, 
            // manga/plugins.rs
            manga::manga_search, manga::get_manga_chapters, manga::get_manga_plugin_names, manga::get_manga_pages, manga::add_manga_plugin,
            // ln/library.rs
            ln::get_ln_lib, ln::add_to_ln_lib, ln::remove_from_ln_lib, ln::update_ln_lib, 
            // ln/plugins.rs
            ln::ln_search, ln::get_ln_chapters, ln::get_ln_plugin_names, ln::get_ln_pages, ln::add_ln_plugin,
            // settings.rs
            update_settings, get_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn fetch(url: String) -> String {
  let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
  let client = reqwest::Client::new();
  let response = client.get(url)
      .header(reqwest::header::USER_AGENT, user_agent)
      .send()
      .await.unwrap();
  let body = response.text().await.unwrap();
  body
}

#[tauri::command]
async fn post_fetch(url: String) -> String {
  let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
  let client = reqwest::Client::new();
  let response = client.post(url)
      .header(reqwest::header::USER_AGENT, user_agent)
      .send()
      .await.unwrap();
  let body = response.text().await.unwrap();
  body
}
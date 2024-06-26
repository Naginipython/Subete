use std::path::PathBuf;
use lazy_static::lazy_static;
use settings::*;
use tauri_plugin_http::reqwest;
// use tauri::Manager;

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
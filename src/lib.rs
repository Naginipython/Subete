use std::{fs::File, io::{Read, Write}, path::PathBuf};
use lazy_static::lazy_static;
use settings::*;
use tauri_plugin_http::reqwest;
use tauri_plugin_sql::{Migration, MigrationKind};

mod manga;
mod ln;
mod settings;

lazy_static! {
    pub static ref FILE_PATH: String = {
      let data_folder: PathBuf = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("./"));
      let mut str = data_folder.to_str().unwrap_or_default().to_string();
      let os = std::env::consts::OS;
      if os == "windows" {
        str.push_str("\\subete");
      } else if os == "android" {
        str = "/data/data/com.subete.dev/files/".to_string();
      } else {
        str.push_str("/subete");
      }
      std::fs::create_dir_all(&str).unwrap_or_default();
      str
    };
  }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let migrations = vec![
    Migration {
      version: 1,
      description: "create_initial_table",
      sql: "CREATE TABLE IF NOT EXISTS manga_library (
        id VARCHAR(255) PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        img VARCHAR(255) NOT NULL,
        plugin VARCHAR(255) NOT NULL,
        authors VARCHAR(255) NOT NULL,
        artists VARCHAR(255) NOT NULL,
        description TEXT
      );",
      kind: MigrationKind::Up,
    },
    Migration {
      version: 1,
      description: "create_manga_chapters_table",
      sql: "CREATE TABLE IF NOT EXISTS manga_chapters (
        id VARCHAR(255) PRIMARY KEY,
        manga_id VARCHAR(255),
        num INT,
        title VARCHAR(255),
        page INT,
        completed BOOLEAN,
        FOREIGN KEY (manga_id) REFERENCES manga_library(id)
      );",
      kind: MigrationKind::Up,
    }
  ];

  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(
      tauri_plugin_sql::Builder::default()
      .add_migrations("sqlite:database.db", migrations)
      .build())
    .invoke_handler(tauri::generate_handler![
        test_read, test_write,
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

#[tauri::command]
fn test_write() {
  let mut file = File::create("/data/data/com.subete.dev/files/test.txt").unwrap();
  let test = "test".as_bytes();
  file.write_all(test).unwrap();
}
#[tauri::command]
fn test_read() -> String {
  let mut file = File::open("/data/data/com.subete.dev/files/test.txt").unwrap();
  let mut buf: Vec<u8> = Vec::new();
  file.read_to_end(&mut buf).unwrap();
  String::from_utf8(buf).expect("");
  String::from(dirs::data_local_dir().unwrap().to_str().unwrap())
}
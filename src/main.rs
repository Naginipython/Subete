// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use lazy_static::lazy_static;
// use manga::*;
use settings::*;
use manga::library::*;
use manga::plugins::*;

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

fn main() {
  // geckodriver
  // let _driver = Command::new("geckodriver").spawn().map_err(|e| format!("Failed to start Geckodriver: {}", e)).expect("Error: Couldn't start Geckodriver");
  jstime_core::init(None);

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      // library.rs
      get_manga_lib, add_to_manga_lib, remove_from_manga_lib, update_manga_lib, 
      // plugins.rs
      manga_search, get_manga_chapters, get_manga_plugin_names, get_manga_pages, add_manga_plugin,
      // settings.rs
      update_settings, get_settings
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
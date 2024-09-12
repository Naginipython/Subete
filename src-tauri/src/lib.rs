use std::{path::PathBuf, sync::LazyLock};
use settings::*;

pub use common::*;

mod anime;
mod manga;
mod ln;
mod common;
mod helpers;
mod settings;

static FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut data_folder: PathBuf = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("./"));
    // let mut str = data_folder.to_str().unwrap_or_default().to_string();
    let os = std::env::consts::OS;
    // if os == "windows" {
    //   data_folder.join("\\subete");
    if os == "android" {
        data_folder = PathBuf::from("/data/data/com.subete.app/files/");
    } else {
        data_folder.push("subete");
    }
    std::fs::create_dir_all(&data_folder).unwrap_or_default();
    data_folder
});

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| {
            // let window = app.get_webview_window("main").unwrap();
            // window.set_fullscreen(true);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            helpers::fetch, helpers::post_fetch,
            // manga/library.rs
            manga::get_manga_lib, manga::add_to_manga_lib, manga::remove_from_manga_lib, manga::update_manga_lib, manga::delete_manga_lib,
            // manga/plugins.rs
            manga::manga_search, manga::get_manga_chapters, manga::get_manga_plugin_names, manga::get_manga_pages, manga::add_manga_plugin, 
            manga::delete_manga_plugins, manga::delete_manga_plugin,
            // manga/update.rs
            manga::save_manga_updates_list, manga::get_manga_updates_list,
            // manga/history.rs
            manga::save_manga_history, manga::get_manga_history,

            // anime/library.rs
            anime::get_anime_lib, anime::add_to_anime_lib, anime::remove_from_anime_lib, anime::update_anime_lib, anime::delete_anime_lib,
            // anime/plugins.rs
            anime::anime_search, anime::get_anime_episodes, anime::get_anime_plugin_names, anime::get_anime_video, anime::add_anime_plugin,
            anime::delete_anime_plugins, anime::delete_anime_plugin,
            
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
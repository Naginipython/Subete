use settings::*;
use tauri::path::BaseDirectory;
use tauri_plugin_sql::Migration;
use std::{path::PathBuf, sync::LazyLock};

pub use common::*;

mod anime;
mod common;
mod helpers;
mod ln;
mod manga;
mod settings;

static FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut data_folder: PathBuf = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("./"));
    let mut test = BaseDirectory::LocalData;
    println!("{:?}", test.variable());
    let os = std::env::consts::OS;
    if os == "android" {
        data_folder = PathBuf::from("/data/data/com.subete.app/files/");
    } else {
        data_folder.push("subete");
    }
    std::fs::create_dir_all(&data_folder).unwrap_or_default();
    data_folder
});
static DOWNLOADS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut downloads_folder = dirs::download_dir().unwrap_or_else(|| PathBuf::from("./"));
    let os = std::env::consts::OS;
    if os == "linux" || os == "windows" {
        downloads_folder.push("subete");
    }
    std::fs::create_dir_all(&downloads_folder).unwrap_or_default();
    downloads_folder
});

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec! [
        Migration {
            version: 1,
            description: "create_manga_library_table",
            sql: "CREATE TABLE manga_library (
                id VARCHAR(255), 
                title VARCHAR(255), 
                img VARCHAR(255),
                plugin VARCHAR(255), 
                authors VARCHAR(255),
                artists VARCHAR(255),
                description VARCHAR(255)
            );",
            kind: tauri_plugin_sql::MigrationKind::Up
        }
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:subete_database.db", migrations)
                .build()
        )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| {
            // let window = app.get_webview_window("main").unwrap();
            // window.set_fullscreen(true);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            helpers::fetch,
            helpers::post_fetch,
            // manga/library.rs
            manga::get_manga_lib,
            manga::add_to_manga_lib,
            manga::remove_from_manga_lib,
            manga::update_manga_lib,
            manga::delete_manga_lib,
            // manga/plugins.rs
            manga::manga_search,
            manga::get_manga_chapters,
            manga::get_manga_plugin_names,
            manga::get_manga_pages,
            manga::add_manga_plugin,
            manga::delete_manga_plugins,
            manga::delete_manga_plugin,
            // manga/update.rs
            manga::save_manga_updates_list,
            manga::get_manga_updates_list,
            // manga/history.rs
            manga::save_manga_history,
            manga::get_manga_history,
            // anime/library.rs
            anime::get_anime_lib,
            anime::add_to_anime_lib,
            anime::remove_from_anime_lib,
            anime::update_anime_lib,
            anime::delete_anime_lib,
            anime::open_in_vlc,
            // anime/plugins.rs
            anime::anime_search,
            anime::get_anime_episodes,
            anime::get_anime_plugin_names,
            anime::get_anime_video,
            anime::add_anime_plugin,
            anime::delete_anime_plugins,
            anime::delete_anime_plugin,
            anime::download_anime,
            // anime/update.rs
            anime::save_anime_updates_list,
            anime::get_anime_updates_list,
            // ln/library.rs
            ln::get_ln_lib,
            ln::add_to_ln_lib,
            ln::remove_from_ln_lib,
            ln::update_ln_lib,
            ln::delete_ln_lib,
            // ln/plugins.rs
            ln::ln_search,
            ln::get_ln_chapters,
            ln::get_ln_plugin_names,
            ln::get_ln_pages,
            ln::add_ln_plugin,
            ln::delete_ln_plugins,
            ln::delete_ln_plugin,
            ln::download_ln,
            // manga/update.rs
            ln::save_ln_updates_list,
            ln::get_ln_updates_list,
            // settings.rs
            update_settings,
            get_settings,
            delete_settings
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
    Anime,
}

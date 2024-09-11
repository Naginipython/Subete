use std::{collections::HashMap, fs::File, path::PathBuf, sync::{LazyLock, Mutex}};
use quickjs_rs::JsValue;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::{append_values, fetch, js_value_to_serde_json, save, Media, FILE_PATH};

static PLUGINS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = (*FILE_PATH).clone();
    path.push("anime_plugins.json");
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
    pub episodes_url: String,
    pub get_episodes: String,
    pub episodes_extra: Value,
    pub videos_url: String,
    pub get_videos: String,
    pub videos_extra: Value,
}

fn replace_url(url: &str, placeholder: &str, replace: &str) -> String {
    url.replace(placeholder, replace)
}
fn get_plugins() -> Vec<Plugins> {
    match File::open(&*PLUGINS_PATH) {
        Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
        Err(_e) => {
            let plugins = vec![];
            save(&*PLUGINS_PATH, &plugins);
            plugins
        },
    }
}

#[tauri::command]
pub fn add_anime_plugin(new_plugin: Plugins) {
    println!("Adding anime plugin...");
    let mut plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    if !names.contains(&new_plugin.id) {
        plugins.push(new_plugin);
        save(&*PLUGINS_PATH, &plugins);
    }
}

#[tauri::command]
pub fn get_anime_plugin_names() -> Value {
    println!("Getting Anime Plugin Names...");
    let plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    json!(names)
}

#[tauri::command]
pub async fn anime_search(query: String, sources: Vec<String>) -> Value {
    println!("Searching Anime(s)...");
    let mut result: Value = json!([]);
    let plugins = get_plugins();
    for p in plugins {
        if sources.contains(&p.id) {
            // Fetching page data
            let url = replace_url(&p.search_url, "{title}", &query);
            let html = fetch(url).await;

            // Getting from plugins
            let search = (p.search).to_string();
            let search1 = format!("{}search(`{}`);", &search, &html);
            
            let context = quickjs_rs::Context::new().unwrap();
            let value = context.eval(&search1).unwrap_or_else(|error| {
                println!("{error}");
                let search2 = format!("{}search(JSON.stringify({}));", &search, &html);
                match context.eval(&search2) {
                    Ok(v) => v,
                    Err(e) => {
                      let h = HashMap::from([(String::from("error"), JsValue::String(format!("{:?} experienced an issue: {e}", p.id)))]);
                      JsValue::Object(h)
                    }
                }
            });
            let r = js_value_to_serde_json(value);
            append_values(&mut result, r)
        }
    }
    result
}

// #[tauri::command]
// pub async fn get_anime_chapters(anime: LibraryItem) -> Value {
//     println!("Getting manga chapters...");
//     let mut result: Value = json!({});
//     let plugins = get_plugins();
//     let plugin = plugins.iter().find(|p| p.id == manga.plugin);
//     if let Some(p) = plugin {
//         // let temp = json!({"url": replace_url(&p.chapters_url, "{id}", &manga.id), "getChapters": (p.get_chapters).to_string()});
//         // result = temp;
//         // Fetching page data
//         let url = replace_url(&p.chapters_url, "{id}", &manga.id);
//         let html = if p.chapters_extra.get("request").is_some() {
//             post_fetch(url).await
//         } else {
//             fetch(url).await
//         };
        
//         let mut chap_code = (&p.get_chapters).clone();
//         chap_code.push_str(&format!("getChapters(JSON.parse({:?}), `{html}`);", serde_json::to_string(&manga).unwrap()));
        
//         let context = quickjs_rs::Context::new().unwrap();
//         let value = match context.eval(&chap_code) {
//             Ok(v) => v,
//             Err(e) => {
//                 println!("{e}");
//                 let h = HashMap::from([(String::from("error"), JsValue::String(format!("{:?} experienced an issue: {e}", p.id)))]);
//                 JsValue::Object(h)
//             }
//         };
//         result = js_value_to_serde_json(value);
//     }
//     result
// }
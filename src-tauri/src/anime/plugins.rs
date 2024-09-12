use std::{fs::File, path::PathBuf, sync::{LazyLock, Mutex}};
use quickjs_runtime::{builder::QuickJsRuntimeBuilder, jsutils::Script, values::JsValueConvertable};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::{fetch, post_fetch, save, Media, FILE_PATH};

use super::LibraryItem;

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

            let runtime = QuickJsRuntimeBuilder::new().build();
            let script = Script::new("search.js", &search);
            runtime.eval_sync(None, script).ok().expect("script failed");
            result = runtime.invoke_function_sync(None, &[], "search", vec![html.to_js_value_facade()]).unwrap().to_serde_value().await.unwrap();
        }
    }
    result
}

#[tauri::command]
pub async fn get_anime_episodes(anime: LibraryItem) -> Value {
    println!("Getting anime episodes...");
    let mut result: Value = json!({});
    let plugins = get_plugins();
    let plugin = plugins.iter().find(|p| p.id == anime.plugin);
    if let Some(p) = plugin {
        // fetch
        let url = replace_url(&p.episodes_url, "{id}", &anime.id);
        let html = if p.episodes_extra.get("request").is_some() {
            post_fetch(url).await
        } else {
            fetch(url).await
        };
        
        // setup code
        let code = (&p.get_episodes).clone();
        // let ep_code = format!("{code} getEpisodes(JSON.parse({:?}), `{html}`);", serde_json::to_string(&anime).unwrap());

        let runtime = QuickJsRuntimeBuilder::new().build();
        let script = Script::new("get_episodes.js", &code);
        runtime.eval_sync(None, script).ok().expect("script failed");
        result = runtime
            .invoke_function_sync(None, &[], "getEpisodes", vec![serde_json::to_string(&anime).unwrap().to_js_value_facade(), html.to_js_value_facade()])
            .unwrap().to_serde_value().await.unwrap();

        // extra
        if let Some(e) = result["extra"].as_object() {
            if let Some(next) = e.get("next") {
                let link = String::from(next.as_str().unwrap());
                let html = fetch(link).await;
                // If there is a next link, call it
                // let ep_code = format!("{code} next(JSON.parse({:?}), `{html}`);", serde_json::to_string(&anime).unwrap());

                result = runtime
                    .invoke_function_sync(None, &[], "next", vec![serde_json::to_string(&result).unwrap().to_js_value_facade(), html.to_js_value_facade()])
                    .unwrap().to_serde_value().await.unwrap();
            //     if let None = episode_result.extra { break; }
            }
        }
    }
    result
}
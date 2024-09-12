use crate::{fetch, post_fetch, Media, FILE_PATH};
use quickjs_runtime::{builder::QuickJsRuntimeBuilder, jsutils::Script, values::JsValueConvertable};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs::File, io::Write, path::PathBuf, sync::{LazyLock, Mutex}};

use super::LibraryItem;

static PLUGINS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = (*FILE_PATH).clone();
    path.push("manga_plugins.json");
    path
});
static PLUGINS: LazyLock<Mutex<Vec<Plugins>>> = LazyLock::new(|| match File::open(&*PLUGINS_PATH) {
    Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
    Err(_e) => {
        File::create(&*PLUGINS_PATH).unwrap();
        Mutex::new(Vec::new())
    }
});

fn save(lib: &Vec<Plugins>) {
    let mut file = File::create(&*PLUGINS_PATH).unwrap();
    let json = serde_json::to_string(lib).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
fn get_plugins() -> Vec<Plugins> {
    match File::open(&*PLUGINS_PATH) {
        Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
        Err(_e) => init_plugins(),
    }
}
#[tauri::command]
pub fn add_manga_plugin(new_plugin: Plugins) {
    println!("Adding manga plugin...");
    let mut plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    if !names.contains(&new_plugin.id) {
        plugins.push(new_plugin);
        save(&plugins);
    }
}
fn init_plugins() -> Vec<Plugins> {
    File::create(&*PLUGINS_PATH).unwrap();
    let plugins = vec![Plugins {
        id: String::from("MangaDex"),
        media_type: Media::Manga,
        search_url: String::from("https://api.mangadex.org/manga?limit=100&includes[]=cover_art&includes[]=author&includes[]=artist&title={title}"),
        // TODO: go through every page
        search: String::from("function search(html) { html = JSON.parse(html); let data = []; for (let d of html['data']) { let temp = {}; temp['id'] = d['id']; if (d['attributes']['title'].hasOwnProperty('en')) { temp['title'] = d['attributes']['title']['en']; } else if (d['attributes']['title'].hasOwnProperty('ja-ro')) { temp['title'] = d['attributes']['title']['ja-ro']; } else { temp['title'] = d['attributes']['title']['ja']; } let filetemp = d['relationships'].filter(o => o.type == 'cover_art')[0]; if (filetemp != undefined) { temp['img'] = `https://uploads.mangadex.org/covers/${temp['id']}/${filetemp['attributes']['fileName']}`; } else { temp['img'] = ''; } temp['plugin'] = 'MangaDex'; temp['description'] = d['attributes']['description']['en']? d['attributes']['description']['en'] : ''; temp['chapters'] = []; let author_names = d['relationships'].filter(x => x.type == 'author').map(y => y['attributes']['name']); let artist_names = d['relationships'].filter(x => x.type == 'artist').map(y => y['attributes']['name']); temp['authors'] = author_names.join(', '); temp['artists'] = artist_names.join(', '); data.push(temp); } return data; }"),
        search_extra: json!({}),
        chapters_url: String::from("https://api.mangadex.org/manga/{id}/feed?limit=500&order[chapter]=asc&translatedLanguage[]=en"),
        get_chapters: String::from("function getChapters(json, html) { json = JSON.parse(json); html = JSON.parse(html); json.chapters = html['data'].map(e => { return { number: parseFloat(e['attributes']['chapter'])? parseFloat(e['attributes']['chapter']) : 0.0, id: e['id'], title: e['attributes']['title'] == '' || e['attributes']['title'] == null? `Chapter ${e['attributes']['chapter']}` : e['attributes']['title'], page: 1, completed: false } }); return json; }"),
        chapters_extra: json!({}),
        pages_url: String::from("https://api.mangadex.org/at-home/server/{id}"),
        get_pages: String::from("function getChapterPages(html) { html = JSON.parse(html); let hash = html['chapter']['hash']; let data = html['chapter']['data']; return data.map(x => `https://uploads.mangadex.org/data/${hash}/${x}`); }"),
        pages_extra: json!({})
    }];
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
pub fn get_manga_plugin_names() -> Value {
    println!("Getting Manga Plugin Names...");
    let plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    json!(names)
}

fn replace_url(url: &str, placeholder: &str, replace: &str) -> String {
    url.replace(placeholder, replace)
}

#[tauri::command]
pub async fn manga_search(query: String, sources: Vec<String>) -> Value {
    println!("Searching Manga(s)...");
    let mut result: Value = json!([]);
    let plugins = get_plugins();
    for p in plugins {
        if sources.contains(&p.id) {
            // Fetching page data
            let url = replace_url(&p.search_url, "{title}", &query);
            let html = fetch(url).await;

            // Getting from plugins
            let search = (p.search).to_string();
            // let search1 = format!("{}search(`{}`);", &search, &html);
            
            // let context = quickjs_rs::Context::new().unwrap();
            // let value = context.eval(&search1).unwrap_or_else(|error| {
            //     println!("{error}");
            //     let search2 = format!("{}search(JSON.stringify({}));", &search, &html);
            //     match context.eval(&search2) {
            //         Ok(v) => v,
            //         Err(e) => {
            //           let h = HashMap::from([(String::from("error"), JsValue::String(format!("{:?} experienced an issue: {e}", p.id)))]);
            //           JsValue::Object(h)
            //         }
            //     }
            // });
            
            let runtime = QuickJsRuntimeBuilder::new().build();
            let script = Script::new("search.js", &search);
            runtime.eval_sync(None, script).ok().expect("script failed");
            result = runtime
                .invoke_function_sync(None, &[], "search", vec![html.to_js_value_facade()])
                .unwrap().to_serde_value().await.unwrap();
        }
    }
    result
}

#[tauri::command]
pub async fn get_manga_chapters(manga: LibraryItem) -> Value {
    println!("Getting manga chapters...");
    let mut result: Value = json!({});
    let plugins = get_plugins();
    let plugin = plugins.iter().find(|p| p.id == manga.plugin);
    if let Some(p) = plugin {
        // let temp = json!({"url": replace_url(&p.chapters_url, "{id}", &manga.id), "getChapters": (p.get_chapters).to_string()});
        // result = temp;
        // Fetching page data
        let url = replace_url(&p.chapters_url, "{id}", &manga.id);
        let html = if p.chapters_extra.get("request").is_some() {
            post_fetch(url).await
        } else {
            fetch(url).await
        };
        
        let chap_code = (&p.get_chapters).clone();
        // chap_code.push_str(&format!("getChapters(JSON.parse({:?}), `{html}`);", serde_json::to_string(&manga).unwrap()));
        
        // let context = quickjs_rs::Context::new().unwrap();
        // let value = match context.eval(&chap_code) {
        //     Ok(v) => v,
        //     Err(e) => {
        //         println!("{e}");
        //         let h = HashMap::from([(String::from("error"), JsValue::String(format!("{:?} experienced an issue: {e}", p.id)))]);
        //         JsValue::Object(h)
        //     }
        // };
        // result = js_value_to_serde_json(value);
        let runtime = QuickJsRuntimeBuilder::new().build();
        let script = Script::new("chapters.js", &chap_code);
        runtime.eval_sync(None, script).ok().expect("script failed");
        result = runtime
            .invoke_function_sync(None, &[], "getChapters", vec![serde_json::to_string(&manga).unwrap().to_js_value_facade(), html.to_js_value_facade()])
            .unwrap().to_serde_value().await.unwrap();
    }
    result
}

#[tauri::command]
pub async fn get_manga_pages(source: String, id: String) -> Value {
    println!("Getting manga pages...");
    let mut result: Value = json!({});
    let plugins = get_plugins();
    let plugin = plugins.iter().find(|p| p.id == source);
    if let Some(p) = plugin {
        // let temp = json!({"url": replace_url(&p.pages_url, "{id}", &id), "getChapterPages": (p.get_pages).to_string()});
        // result = temp;
        let url = replace_url(&p.pages_url, "{id}", &id);
        let html = fetch(url).await;
        
        let chap_code = (&p.get_pages).clone();
        // chap_code.push_str(&format!("getChapterPages(`{html}`);"));
        
        // let context = quickjs_rs::Context::new().unwrap();
        // let value = context.eval(&chap_code).unwrap();
        // result = js_value_to_serde_json(value);
        let runtime = QuickJsRuntimeBuilder::new().build();
        let script = Script::new("pages.js", &chap_code);
        runtime.eval_sync(None, script).ok().expect("script failed");
        result = runtime
            .invoke_function_sync(None, &[], "getChapterPages", vec![html.to_js_value_facade()])
            .unwrap().to_serde_value().await.unwrap();
    }
    result
}

#[tauri::command]
pub fn delete_manga_plugin(plugin: String) {
    println!("Deleting manga plugin: {plugin}");
    let mut plugins = PLUGINS.lock().unwrap();
    plugins.retain(|p| p.id != plugin);
    save(&plugins);
}

#[tauri::command]
pub fn delete_manga_plugins() {
    println!("Deleting manga plugins...");
    let mut plugins = PLUGINS.lock().unwrap();
    *plugins = init_plugins();
    std::fs::remove_file(&*PLUGINS_PATH).unwrap();
}
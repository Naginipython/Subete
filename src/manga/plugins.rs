use crate::FILE_PATH;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs::File, io::Write, sync::Mutex};
// use tauri::{api::http::{ClientBuilder, HttpRequestBuilder, ResponseType}, http::header::USER_AGENT};
use lazy_static::lazy_static;
use tauri_plugin_http::reqwest;

use super::{ChapterItem, LibraryItem};

lazy_static! {
    pub static ref PLUGINS_PATH: String = {
        let mut path = (*FILE_PATH).clone();
        path.push_str("/manga_plugins.json");
        path
    };
    static ref MANGA_PLUGINS: Mutex<Vec<Plugins>> = match File::open(&*PLUGINS_PATH) {
        Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
        Err(_e) => {
            let plugins = init_manga_plugins();
            Mutex::new(plugins)
        }
    };
}

fn save(lib: &Vec<Plugins>) {
    let mut file = File::create(&*PLUGINS_PATH).unwrap();
    let json = serde_json::to_string(&*lib).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
fn get_plugins() -> Vec<Plugins> {
    match File::open(&*PLUGINS_PATH) {
        Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
        Err(_e) => init_manga_plugins(),
    }
}
#[tauri::command]
pub fn add_manga_plugin(new_plugin: Plugins) {
    println!("Adding plugin...");
    let mut plugins = MANGA_PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    if !names.contains(&new_plugin.id) {
        plugins.push(new_plugin);
        save(&*plugins);
    }
}
pub fn init_manga_plugins() -> Vec<Plugins> {
    File::create(&*PLUGINS_PATH).unwrap();
    let plugins = vec![Plugins {
    id: String::from("MangaDex"),
    media_type: Media::Manga,
    search_url: String::from("https://api.mangadex.org/manga?limit=100&includes[]=cover_art&includes[]=author&includes[]=artist&title={title}"),
    // TODO: go through every page
    search: String::from("function search(json) { json = JSON.parse(json); let data = []; for (let d of json['data']) { let temp = {}; temp['id'] = d['id']; temp['title'] = d['attributes']['title']['en']; let filetemp = d['relationships'].filter(o => o.type == 'cover_art')[0]; temp['img'] = `https://uploads.mangadex.org/covers/${temp['id']}/${filetemp['attributes']['fileName']}`; temp['plugin'] = 'MangaDex'; temp['description'] = d['attributes']['description']['en']; temp['chapters'] = []; let author_names = d['relationships'].filter(x => x.type == 'author').map(y => y['attributes']['name']); let artist_names = d['relationships'].filter(x => x.type == 'artist').map(y => y['attributes']['name']); temp['authors'] = author_names.join(', '); temp['artists'] = artist_names.join(', '); data.push(temp); } return data;}"),
    // TODO: add search_extra. This will do a second fetch if needed
    chapters_url: String::from("https://api.mangadex.org/manga/{id}/feed?limit=500&order[chapter]=asc&translatedLanguage[]=en"),
    get_chapters: String::from("function getChapters(json) { json = JSON.parse(json); return json['data'].map(e => { return { number: parseFloat(e['attributes']['chapter'])? parseFloat(e['attributes']['chapter']) : 0.0, id: e['id'], title: e['attributes']['title'] == '' || e['attributes']['title'] == null? `Chapter ${e['attributes']['chapter']}` : e['attributes']['title'], page: 1, completed: false } }); }"),
    pages_url: String::from("https://api.mangadex.org/at-home/server/{id}"),
    get_pages: String::from("function getChapterPages(json) { json = JSON.parse(json); let hash = json['chapter']['hash']; let data = json['chapter']['data']; return data.map(x => `https://uploads.mangadex.org/data/${hash}/${x}`); }"),
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
    pub chapters_url: String,
    pub get_chapters: String,
    pub pages_url: String,
    pub get_pages: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Media {
    #[serde(rename = "manga")]
    Manga, 
    #[serde(rename = "ln")]
    Ln, 
    #[serde(rename = "anime")]
    Anime
}
impl Default for Media {
    fn default() -> Self { Media::Manga }
}

#[tauri::command]
pub fn get_manga_plugin_names() -> Value {
    println!("Getting Plugin Names...");
    let plugins = MANGA_PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    json!(names)
}

fn replace_url(url: &String, placeholder: &str, replace: &str) -> String {
    url.replace(placeholder, replace)
}
async fn fetch(url: String) -> String {
    // Fetching page data
    // let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
    // let client = ClientBuilder::new().max_redirections(3).build().unwrap();
    // let request = HttpRequestBuilder::new("GET", url)
    //     .unwrap()
    //     .header(USER_AGENT, user_agent)
    //     .unwrap()
    //     .response_type(ResponseType::Text);

    // Sends the request and gets the data
    // let res = client.send(request).await.unwrap();
    // res.read().await.unwrap().data
    let body = reqwest::get(url)
      .await.unwrap()
      .text()
      .await.unwrap();
    body
}

#[tauri::command]
pub async fn manga_search(query: String, sources: Vec<String>) -> Value {
    println!("Searching...");
    let mut result: Vec<LibraryItem> = Vec::new();
    let plugins = get_plugins();
    for p in plugins {
        if sources.contains(&p.id) {
            // Fetching page data
            let url = replace_url(&p.search_url, "{title}", &query);
            let data = fetch(url).await;

            // Getting from plugins
            let mut search = (p.search).to_string();
            search.push_str(&format!("JSON.stringify(search({}))", data));

            let mut scope = jstime_core::JSTime::new(jstime_core::Options::default());
            let output = scope.run_script(&search, "jstime").expect("");
            let mut r: Vec<LibraryItem> = serde_json::from_str(&output).unwrap();
            result.append(&mut r);
        }
    }
    json!(result)
}

#[tauri::command]
pub async fn get_manga_chapters(source: String, id: String) -> Value {
    println!("Getting chapters...");
    let mut result: Vec<ChapterItem> = Vec::new();
    let plugins = get_plugins();
    let plugin = plugins.iter().find(|p| p.id == source);
    if let Some(p) = plugin {
        let url = replace_url(&p.chapters_url, "{id}", &id);
        let data = fetch(url).await;

        // Getting from plugins
        let mut chap_func = (p.get_chapters).to_string();
        chap_func.push_str(&format!("JSON.stringify(getChapters({}))", data));

        let mut scope = jstime_core::JSTime::new(jstime_core::Options::default());
        let output = scope
            .run_script(&chap_func, "jstime")
            .expect("JS Somehow failed");
        let test: Value = serde_json::from_str(&output).unwrap();
        println!("{:?}", test);
        result = serde_json::from_str(&output).unwrap();
    }
    json!(result)
}

#[tauri::command]
pub async fn get_manga_pages(source: String, id: String) -> Value {
    println!("Getting pages...");
    let mut result: Vec<String> = Vec::new();
    let plugins = get_plugins();
    let plugin = plugins.iter().find(|p| p.id == source);
    if let Some(p) = plugin {
        let url = replace_url(&p.pages_url, "{id}", &id);
        let data = fetch(url).await;

        // Getting from plugins
        let mut pages_func = (p.get_pages).to_string();
        pages_func.push_str(&format!("JSON.stringify(getChapterPages({}))", data));

        let mut scope = jstime_core::JSTime::new(jstime_core::Options::default());
        let output = scope
            .run_script(&pages_func, "jstime")
            .expect("JS Somehow failed");
        result = serde_json::from_str(&output).unwrap();
    }
    json!(result)
}

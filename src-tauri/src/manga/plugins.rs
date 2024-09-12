use std::{fs::File, path::PathBuf, sync::{LazyLock, Mutex}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::{get_item, get_list, save, search, IsPlugin, Media, FILE_PATH};

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
impl IsPlugin for Plugins {
    fn id(&self) -> String { self.id.clone() }
    fn search_url(&self) -> &str { &self.search_url }
    fn search(&self) -> &str { &self.search }
    fn list_url(&self) -> &str { &self.chapters_url }
    fn get_list(&self) -> &str { &self.get_chapters }
    fn list_extra(&self) -> &Value { &self.chapters_extra }
    fn item_url(&self) -> &str { &self.pages_url }
    fn get_item(&self) -> &str { &self.get_pages }
    fn item_fn(&self) -> &str { "getChapterPages" }
}

fn get_plugins() -> Vec<Plugins> {
    match File::open(&*PLUGINS_PATH) {
        Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
        Err(_e) => init_plugins(),
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
    save(&*PLUGINS_PATH, &plugins);
    plugins
}

#[tauri::command]
pub fn add_manga_plugin(new_plugin: Plugins) {
    println!("Adding manga plugin...");
    let mut plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    if !names.contains(&new_plugin.id) {
        plugins.push(new_plugin);
        save(&*PLUGINS_PATH, &plugins);
    }
}

#[tauri::command]
pub fn get_manga_plugin_names() -> Value {
    println!("Getting Manga Plugin Names...");
    let plugins = PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    json!(names)
}

#[tauri::command]
pub async fn manga_search(query: String, sources: Vec<String>) -> Value {
    search("manga", get_plugins(), query, sources).await
}

#[tauri::command]
pub async fn get_manga_chapters(manga: LibraryItem) -> Value {
    get_list("manga", get_plugins(), manga).await
}

#[tauri::command]
pub async fn get_manga_pages(source: String, id: String) -> Value {
    get_item("manga", get_plugins(), source, id).await
}

#[tauri::command]
pub fn delete_manga_plugin(plugin: String) {
    println!("Deleting manga plugin: {plugin}");
    let mut plugins = PLUGINS.lock().unwrap();
    plugins.retain(|p| p.id != plugin);
    save(&*PLUGINS_PATH, &plugins);
}

#[tauri::command]
pub fn delete_manga_plugins() {
    println!("Deleting manga plugins...");
    let mut plugins = PLUGINS.lock().unwrap();
    *plugins = init_plugins();
    std::fs::remove_file(&*PLUGINS_PATH).unwrap();
}
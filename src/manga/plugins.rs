use crate::{fetch, post_fetch, FILE_PATH};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs::File, io::Write, path::PathBuf, sync::Mutex};
use lazy_static::lazy_static;

use super::LibraryItem;

lazy_static! {
    pub static ref PLUGINS_PATH: PathBuf = {
        let mut path = (*FILE_PATH).clone();
        path.push("manga_plugins.json");
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
    let json = serde_json::to_string(lib).unwrap();
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
    println!("Adding Manga plugin...");
    let mut plugins = MANGA_PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    if !names.contains(&new_plugin.id) {
        plugins.push(new_plugin);
        save(&plugins);
    }
}
pub fn init_manga_plugins() -> Vec<Plugins> {
    File::create(&*PLUGINS_PATH).unwrap();
    let plugins = vec![Plugins {
    id: String::from("MangaDex"),
    media_type: Media::Manga,
    search_url: String::from("https://api.mangadex.org/manga?limit=100&includes[]=cover_art&includes[]=author&includes[]=artist&title={title}"),
    // TODO: go through every page
    search: String::from("function search(html) { html = JSON.parse(html); let data = []; for (let d of html['data']) { let temp = {}; temp['id'] = d['id']; temp['title'] = d['attributes']['title']['en']; let filetemp = d['relationships'].filter(o => o.type == 'cover_art')[0]; temp['img'] = `https://uploads.mangadex.org/covers/${temp['id']}/${filetemp['attributes']['fileName']}`; temp['plugin'] = 'MangaDex'; temp['description'] = d['attributes']['description']['en']? d['attributes']['description']['en'] : ''; temp['chapters'] = []; let author_names = d['relationships'].filter(x => x.type == 'author').map(y => y['attributes']['name']); let artist_names = d['relationships'].filter(x => x.type == 'artist').map(y => y['attributes']['name']); temp['authors'] = author_names.join(', '); temp['artists'] = artist_names.join(', '); data.push(temp); } return data; }"),
    search_extra: json!({}),
    chapters_url: String::from("https://api.mangadex.org/manga/{id}/feed?limit=500&order[chapter]=asc&translatedLanguage[]=en"),
    get_chapters: String::from("function getChapters(json, html) { html = JSON.parse(html); json.chapters = html['data'].map(e => { return { number: parseFloat(e['attributes']['chapter'])? parseFloat(e['attributes']['chapter']) : 0.0, id: e['id'], title: e['attributes']['title'] == '' || e['attributes']['title'] == null? `Chapter ${e['attributes']['chapter']}` : e['attributes']['title'], page: 1, completed: false } }); return json; }"),
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
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum Media {
    #[serde(rename = "manga")]
    #[default]
    Manga, 
    #[serde(rename = "ln")]
    Ln, 
    #[serde(rename = "anime")]
    Anime
}
// impl Default for Media {
//     fn default() -> Self { Media::Manga }
// }

#[tauri::command]
pub fn get_manga_plugin_names() -> Value {
    println!("Getting Manga Plugin Names...");
    let plugins = MANGA_PLUGINS.lock().unwrap();
    let names: Vec<String> = plugins.iter().map(|p| p.id.clone()).collect();
    json!(names)
}

fn replace_url(url: &str, placeholder: &str, replace: &str) -> String {
    url.replace(placeholder, replace)
}

#[tauri::command]
pub async fn manga_search(query: String, sources: Vec<String>) -> Value {
    println!("Searching Manga(s)...");
    let mut result: Value = json!({});
    let plugins = get_plugins();
    for p in plugins {
        if sources.contains(&p.id) {
            // Fetching page data
            let url = replace_url(&p.search_url, "{title}", &query);
            let html = fetch(url).await;

            // Getting from plugins
            let search = (p.search).to_string();
            let search1 = format!("{}search(`{}`);", &search, &html);

            let search_value: Value = rustyscript::evaluate(&search1).unwrap_or_else(|_e| {
                let search2 = format!("{}search(JSON.stringify({}));", &search, &html);
                rustyscript::evaluate(&search2).unwrap_or_default()
            });
            result = search_value;
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
        let temp = json!({"url": replace_url(&p.chapters_url, "{id}", &manga.id), "getChapters": (p.get_chapters).to_string()});
        result = temp;
        // Fetching page data
        // let url = replace_url(&p.chapters_url, "{id}", &manga.id);
        // let html = if p.chapters_extra.get("request").is_some() {
        //     post_fetch(url).await
        // } else {
        //     fetch(url).await
        // };
        // // Getting from plugins
        // println!("{:?}", manga);
        // let chap_func = format!("{} getChapters(JSON.parse({:?}), `{html}`)", &p.get_chapters, serde_json::to_string(&manga).unwrap());
        // // println!("{:?}", chap_func);

        // let chap_value: Value = rustyscript::evaluate(&chap_func).unwrap_or_else(|e| {
        //     eprintln!("Error in chapter js: {e}");
        //     serde_json::to_value(&manga).unwrap()
        // });
        // result = chap_value;
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
        let temp = json!({"url": replace_url(&p.pages_url, "{id}", &id), "getChapterPages": (p.get_pages).to_string()});
        result = temp;
    }
    result
}

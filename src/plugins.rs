use std::{fs::File, io::Write, sync::Mutex};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{api::http::{ClientBuilder, HttpRequestBuilder, ResponseType}, http::header::USER_AGENT};
use lazy_static::lazy_static;

use crate::LibraryItem;

lazy_static! {
    static ref PLUGINS: Mutex<Vec<Plugins>> = match File::open("plugins.json") {
      Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
      Err(e) => {
        eprintln!("ERROR: {e}");
        let plugins = init_plugins();
        Mutex::new(plugins)
      }
    };
}

fn save(lib: &Vec<Plugins>) {
    let mut file = File::create("plugins.json").unwrap();
    let json = serde_json::to_string(&*lib).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
fn get_plugins() -> Vec<Plugins> {
    match File::open("plugins.json") {
        Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
        Err(_e) => init_plugins()
    }
}
fn init_plugins() -> Vec<Plugins> {
    let plugins = vec![Plugins {
        id: String::from("mangadex"),
        search_url: String::from("https://api.mangadex.org/manga?limit=100&includes[]=cover_art&includes[]=author&includes[]=artist&title="),
        search: String::from("
          function search(json) {
            let data = [];
            for (let d of json['data']) {
              let temp = {};
              temp['id'] = d['id'];
              temp['title'] = d['attributes']['title']['en'];
              let filetemp = d['relationships'].filter(o => o.type == 'cover_art')[0];
              temp['img'] = `https://uploads.mangadex.org/covers/${temp['id']}/${filetemp['attributes']['fileName']}`;
              temp['extention'] = 'MangaDex';
              temp['description'] = d['attributes']['description']['en'];
              temp['chapters'] = [];
              let author_names = d['relationships'].filter(x => x.type == 'author').map(y => y['attributes']['name']);
              let artist_names = d['relationships'].filter(x => x.type == 'artist').map(y => y['attributes']['name']);
              temp['authors'] = author_names.join(', ');
              temp['artists'] = artist_names.join(', ');
              data.push(temp);
            }
            return data;
          }
        "),
        get_chapters: String::from("
        
        "),
        get_pages: String::from("
        
        "),
    }];
    save(&plugins);
    plugins
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
struct Plugins {
  id: String,
  search_url: String,
  search: String,
  get_chapters: String,
  get_pages: String
}

#[tauri::command]
pub async fn search(query: String, sources: Vec<String>) -> Value {
    let mut result: Vec<LibraryItem> = Vec::new();
    let plugins = get_plugins();
    for p in plugins {
        if sources.contains(&p.id.to_lowercase()) {
            // Fetching page data
            let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
            let mut url = p.search_url;
            url.push_str(&query);
            let client = ClientBuilder::new()
                .max_redirections(3)
                .build().unwrap();
            let request = HttpRequestBuilder::new("GET", url).unwrap()
                .header(USER_AGENT, user_agent).unwrap()
                .response_type(ResponseType::Text);
            
            // Sends the request and gets the data
            let res = client.send(request).await.unwrap();
            let data = res.read().await.unwrap().data;
    
            // Getting from plugins
          let mut search = (p.search).to_string();
          search.push_str(&format!("JSON.stringify(search(JSON.parse({})))", data));
        
          jstime_core::init(None);
          let mut scope = jstime_core::JSTime::new(jstime_core::Options::default());
          let output = scope.run_script(&search, "jstime").expect("");
          result = serde_json::from_str(&output).unwrap();
        }
  }
  json!(result)
}
use diesel::{Identifiable, Insertable, Queryable};
use diesel::prelude::*;
use serde_json::{json, Value};
use tauri::{api::http::{ClientBuilder, HttpRequestBuilder, ResponseType}, http::header::USER_AGENT};
use crate::database::find_manga;
use crate::{ChapterItem, LibraryItem};
use crate::{database::establish_connection, schema::plugins};

// ----- DATABASE -----
pub fn init_plugins()  {
  match get_plugins() {
    Ok(plugins) => {
      if plugins.len() == 0 {
        let p = Plugin {
          id: String::from("MangaDex"),
          search_url: String::from("https://api.mangadex.org/manga?limit=100&includes[]=cover_art&includes[]=author&includes[]=artist&title={title}"),
          search: String::from("
            function search(json) {
              let data = [];
              for (let d of json['data']) {
                let temp = {};
                temp['id'] = d['id'];
                temp['title'] = d['attributes']['title']['en'];
                let filetemp = d['relationships'].filter(o => o.type == 'cover_art')[0];
                temp['img'] = `https://uploads.mangadex.org/covers/${temp['id']}/${filetemp['attributes']['fileName']}`;
                temp['extension'] = 'MangaDex';
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
          chapters_url: String::from("https://api.mangadex.org/manga/{id}/feed?limit=500&order[chapter]=asc&translatedLanguage[]=en"),
          get_chapters: String::from("
            function getChapters(json) {
              return json['data'].map(e => {
                return {
                    number: parseInt(e['attributes']['chapter']),
                    id: e['id'],
                    title: e['attributes']['title'] == '' || e['attributes']['title'] == null? `Chapter ${e['attributes']['chapter']}` : e['attributes']['title'],
                    page: 1,
                    completed: false
                }
              });
            }
          "),
          pages_url: String::from("https://api.mangadex.org/at-home/server/{id}"),
          get_pages: String::from("
            function getChapterPages(json) {
              let hash = json['chapter']['hash'];
              let data = json['chapter']['data'];
              return data.map(x => `https://uploads.mangadex.org/data/${hash}/${x}`);
            }
          "),
        };
        if let Err(e) = create_plugin(p) {
          eprintln!("ERROR: Couldn't init plugins: {e}")
        }
      }
    },
    Err(e) => eprintln!("ERROR: Couldn't get plugins: {e}")
  }
}
pub fn get_plugins() -> Result<Vec<Plugin>, String> {
  let connection = &mut establish_connection();
  let result = plugins::table.load::<Plugin>(connection).map_err(|err| err.to_string())?;
  Ok(result)
}
pub fn create_plugin(plugin: Plugin) -> Result<(), String> {
  let connection = &mut establish_connection();

  let new_plugin = NewPlugin {
    id: &plugin.id,
    search_url: &plugin.search_url,
    search: &plugin.search,
    chapters_url: &plugin.chapters_url,
    get_chapters: &plugin.get_chapters,
    pages_url: &plugin.pages_url,
    get_pages: &plugin.get_pages
  };
  diesel::insert_into(plugins::table)
      .values(&new_plugin)
      .execute(connection)
      .map_err(|err| err.to_string())?;

  Ok(())
}
#[tauri::command]
pub fn get_plugin_names() -> Vec<String> {
  let connection = &mut establish_connection();
    let results = plugins::table
        .select(plugins::id)
        .load::<String>(connection)
        .map_err(|err| err.to_string()).unwrap_or_default();
    results
}


// ----- PLUGIN ACTIONS -----
fn replace_url(url: &String, placeholder: &str, replace: &str) -> String {
  url.replace(placeholder, replace)
}
async fn fetch(url: String) -> Value {
  // Fetching page data
  let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
  let client = ClientBuilder::new()
      .max_redirections(3)
      .build().unwrap();
  let request = HttpRequestBuilder::new("GET", url).unwrap()
      .header(USER_AGENT, user_agent).unwrap()
      .response_type(ResponseType::Text);
  
  // Sends the request and gets the data
  let res = client.send(request).await.unwrap();
  res.read().await.unwrap().data
}

#[tauri::command]
pub async fn search(query: String, sources: Vec<String>) -> Value {
  let mut result: Vec<LibraryItem> = Vec::new();
  let plugins = get_plugins().unwrap_or_default();
  for p in plugins {
    if sources.contains(&p.id.to_lowercase()) {
      // Fetching page data
      let url = replace_url(&p.search_url, "{title}", &query);
      let data = fetch(url).await;
  
      // Getting from plugins
      let mut search = (p.search).to_string();
      search.push_str(&format!("JSON.stringify(search(JSON.parse({})))", data));
    
      let mut scope = jstime_core::JSTime::new(jstime_core::Options::default());
      let output = scope.run_script(&search, "jstime").expect("");
      result = serde_json::from_str(&output).unwrap();
    }
  }
  json!(result)
}

#[tauri::command]
pub async fn get_chapters(source: String, id: String) -> Value {
  println!("Getting Chapters");
  let mut result: Vec<ChapterItem> = Vec::new();
  let plugins = get_plugins().unwrap_or_default();
  let plugin = plugins.iter().find(|p| p.id == source);
  match find_manga(id) {
    Ok(manga) => {
      if let Some(p) = plugin {
        let url = replace_url(&p.chapters_url, "{id}", &manga.id);
        let data = fetch(url).await;
        
        // Getting from plugins
        let mut chap_func = (p.get_chapters).to_string();
        chap_func.push_str(&format!("JSON.stringify(getChapters(JSON.parse({})))", data));
      
        let mut scope = jstime_core::JSTime::new(jstime_core::Options::default());
        let output = scope.run_script(&chap_func, "jstime").expect("JS Somehow failed");
        println!("test");
        result = serde_json::from_str(&output).unwrap();
      }
    },
    Err(e) => eprintln!("ERROR: Couldn't find Manga. {e}")
  }
  json!(result)
}

// ----- DATA MODELS -----
#[derive(Queryable, Identifiable, diesel::AsChangeset, Debug)]
#[table_name = "plugins"]
pub struct Plugin {
    pub id: String,
    pub search_url: String,
    pub search: String,
    pub chapters_url: String,
    pub get_chapters: String,
    pub pages_url: String,
    pub get_pages: String,
}
#[derive(Insertable)]
#[table_name = "plugins"]
pub struct NewPlugin<'a> {
    pub id: &'a str,
    pub search_url: &'a str,
    pub search: &'a str,
    pub chapters_url: &'a str,
    pub get_chapters: &'a str,
    pub pages_url: &'a str,
    pub get_pages: &'a str,
}
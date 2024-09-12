use std::{fs::File, io::Write, path::PathBuf, sync::Mutex};
use quickjs_runtime::{builder::QuickJsRuntimeBuilder, jsutils::Script, values::JsValueConvertable};
use serde::Serialize;
use serde_json::{json, Value};
use crate::helpers;

// TRAITS
pub trait HasId {
    fn id(&self) -> &str;
}
pub trait IsPlugin {
    fn id(&self) -> String;
    fn search_url(&self) -> &str;
    fn search(&self) -> &str;
    fn list_url(&self) -> &str;
    fn get_list(&self) -> &str;
    fn list_extra(&self) -> &Value;
    fn item_url(&self) -> &str;
    fn get_item(&self) -> &str;
    fn item_fn(&self) -> &str;
}
pub trait IsItem {
    fn plugin(&self) -> &str;
    fn list_fn(&self) -> &str;
}

// LIBRARY
pub fn save<T: Serialize>(path: &PathBuf, data: &Vec<T>) {
    let mut file = File::create(path).unwrap();
    let json = serde_json::to_string(data).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
pub fn get_lib<T: Serialize>(media: &str, lock: &Mutex<Vec<T>>) -> Value {
    // todo: fix unwraps
    println!("Getting {media} library...");
    let lib = lock.lock().unwrap();
    serde_json::to_value(&*lib).unwrap()
}
pub fn add_to_lib<T: Serialize + PartialEq>(media: &str, lock: &Mutex<Vec<T>>, path: &PathBuf, item: T) {
    // todo: fix unwraps
    println!("Adding to {media} library...");
    let mut lib = lock.lock().unwrap();
    if !lib.iter().any(|l| l.eq(&item)) {
        lib.push(item);
        save(path, &lib);
    }
}
pub fn update_lib<T: Serialize + PartialEq>(media: &str, lock: &Mutex<Vec<T>>, path: &PathBuf, item: T) {
    println!("Updating {media} library...");
    let mut lib = lock.lock().unwrap();
    for entry in lib.iter_mut() {
        if item.eq(&entry) {
            *entry = item;
            save(path, &lib);
            return;
        }
    }
    // If it somehow isn't on the list, add it
    lib.push(item);
}
pub fn remove_from_lib<T: Serialize + HasId>(media: &str, lock: &Mutex<Vec<T>>, path: &PathBuf, id: String) {
    // todo: fix unwraps, also add plugin?
    println!("Removing from {media} library...");
    let mut lib = lock.lock().unwrap();
    lib.retain(|l| l.id() != id);
    save(path, &lib);
}
pub fn delete_entire_lib<T: Serialize>(media: &str, lock: &Mutex<Vec<T>>, path: &PathBuf) {
    println!("Deleting {media} lib...");
    let mut lib = lock.lock().unwrap();
    *lib = vec![];
    std::fs::remove_file(path).unwrap();
}

// PLUGINS
fn replace_url(url: &str, placeholder: &str, replace: &str) -> String {
    url.replace(placeholder, replace)
}

pub async fn search<T: Serialize + IsPlugin>(
    media: &str,
    plugins: Vec<T>,
    query: String, 
    sources: Vec<String>
) -> Value {
    println!("Searching {media}(s)...");
    let mut result: Value = json!([]);
    for p in plugins {
        if sources.contains(&p.id()) {
            // Fetching page data
            let url = replace_url(p.search_url(), "{title}", &query);
            let html = helpers::fetch(url).await;

            // Getting from plugins
            let search = (p.search()).to_string();

            let runtime = QuickJsRuntimeBuilder::new().build();
            let script = Script::new("search.js", &search);
            runtime.eval_sync(None, script).ok().expect("script failed");
            result = runtime.invoke_function_sync(None, &[], "search", vec![html.to_js_value_facade()]).unwrap().to_serde_value().await.unwrap();
        }
    }
    result
}

pub async fn get_list<T: Serialize + IsPlugin, U: Serialize + IsItem + HasId>(media: &str, plugins: Vec<T>, item: U) -> Value {
    let m_type = if media=="anime" { "episodes" } else { "chapters" };
    println!("Getting {media} {m_type}..." );
    let mut result: Value = json!({});

    let plugin = plugins.iter().find(|p| p.id() == item.plugin());
    if let Some(p) = plugin {
        // fetch
        let url = replace_url(p.list_url(), "{id}", item.id());
        let html = if p.list_extra().get("request").is_some() {
            helpers::post_fetch(url).await
        } else {
            helpers::fetch(url).await
        };
        
        // setup code
        let code = p.get_list();
        
        let runtime = QuickJsRuntimeBuilder::new().build();
        let script = Script::new("get_list.js", &code);
        runtime.eval_sync(None, script).ok().expect("script failed");
        result = runtime
            .invoke_function_sync(None, &[], item.list_fn(), vec![serde_json::to_string(&item).unwrap().to_js_value_facade(), html.to_js_value_facade()])
            .unwrap().to_serde_value().await.unwrap();

        // extra
        if let Some(e) = result["extra"].as_object() {
            // If there is a next link, call it
            if let Some(next) = e.get("next") {
                let link = String::from(next.as_str().unwrap());
                let html = helpers::fetch(link).await;

                result = runtime
                    .invoke_function_sync(None, &[], "next", vec![serde_json::to_string(&result).unwrap().to_js_value_facade(), html.to_js_value_facade()])
                    .unwrap().to_serde_value().await.unwrap();
                // if let None = episode_result.extra { break; }
            }
        }
    }
    result
}

pub async fn get_item<T: Serialize + IsPlugin>(media: &str, plugins: Vec<T>, source: String, id: String) -> Value {
    let m_type = if media=="anime" { "video" } else { "pages" };
    println!("Getting {media} {m_type}..." );
    let mut result: Value = json!({});
    let plugin = plugins.iter().find(|p| p.id() == source);
    if let Some(p) = plugin {
        let url = replace_url(p.item_url(), "{id}", &id);
        let html = helpers::fetch(url).await;
        
        let code = p.get_item();

        let runtime = QuickJsRuntimeBuilder::new().build();
        let script = Script::new("item.js", &code);
        runtime.eval_sync(None, script).ok().expect("script failed");
        result = runtime
            .invoke_function_sync(None, &[], p.item_fn(), vec![html.to_js_value_facade()])
            .unwrap().to_serde_value().await.unwrap();

        // extra
        let mut count = 0;
        while let Some(next) = result.get("next") {
            count+=1;
            let next_count = if count==1 { "" } else { &format!("{}", count) };
            let html = if next.as_str().unwrap() == "BUILD" {
                // TODO: make more generic
                let iv = helpers::text_to_byte_arr(result["decrypt"]["iv"].as_str().unwrap());
                let key = helpers::text_to_byte_arr(result["decrypt"]["key"].as_str().unwrap());
                let decrypt = helpers::crypto_handler(result["decrypt"]["string"].as_str().unwrap(), &iv[..], &key[..], false).unwrap()
                    .split('&').skip(1).collect::<Vec<_>>().join("&");
                let encrypt = helpers::crypto_handler(result["encrypt"]["string"].as_str().unwrap(), &iv[..], &key[..], true).unwrap();
                let link = result["build"]
                    .as_str()
                    .unwrap()
                    .replace("$encrypt", &encrypt)
                    .replace("$decrypt", &decrypt);
                helpers::fetch_with_header(link, "X-Requested-With", "XMLHttpRequest").await
            } else if next.as_str().unwrap() == "CRYPTO" {
                let iv = helpers::text_to_byte_arr(result["decrypt"]["iv"].as_str().unwrap());
                let key = helpers::text_to_byte_arr(result["decrypt"]["key"].as_str().unwrap());
                let decrypt = helpers::crypto_handler(result["decrypt"]["string"].as_str().unwrap(), &iv[..], &key[..], false).unwrap();
                result = json!({"data": decrypt});
                String::new()
            } else {
                helpers::fetch(String::from(next.as_str().unwrap())).await
            };
            result = runtime
                .invoke_function_sync(None, &[], &format!("next{next_count}"), vec![serde_json::to_string(&result).unwrap().to_js_value_facade(), html.to_js_value_facade()])
                .unwrap().to_serde_value().await.unwrap();
        }
    }
    result
}
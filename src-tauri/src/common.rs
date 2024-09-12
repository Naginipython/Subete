use std::{fs::File, io::Write, path::PathBuf, sync::Mutex};

use quickjs_runtime::{builder::QuickJsRuntimeBuilder, jsutils::Script, values::JsValueConvertable};
use serde::Serialize;
use serde_json::{json, Value};

use crate::fetch;

pub trait HasId {
    fn id(&self) -> &str;
}
pub trait IsPlugin {
    fn id(&self) -> String;
    fn search_url(&self) -> &str;
    fn search(&self) -> &str;
}

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
            let html = fetch(url).await;

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
use crate::FILE_PATH;
use std::{fs::File, io::Write, path::PathBuf, sync::Mutex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref LIB_PATH: PathBuf = {
    let mut path = (*FILE_PATH).clone();
    path.push("ln_library.json");
    path
  };
  pub static ref LIB: Mutex<Vec<LibraryItem>> = match File::open(&*LIB_PATH) {
      Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
      Err(_e) => {
        File::create(&*LIB_PATH).unwrap();
        Mutex::new(Vec::new())
      }
  };
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LibraryItem {
  pub id: String,
  pub title: String,
  pub img: String,
  pub plugin: String,
  pub authors: String,
  pub artists: String,
  pub description: Option<String>,
  pub chapters: Vec<ChapterItem>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChapterItem {
  pub id: String,
  pub number: f64,
  pub title: String,
  pub page: u32,
  pub completed: bool
}

fn save(lib: &Vec<LibraryItem>) {
  let mut file = File::create(&*LIB_PATH).unwrap();
  let json = serde_json::to_string(lib).unwrap();
  file.write_all(json.as_bytes()).unwrap();
}

#[tauri::command]
pub fn get_ln_lib() -> Value {
  // todo: fix unwraps
  println!("Getting light novel library...");
  let lib = LIB.lock().unwrap();
  serde_json::to_value(&*lib).unwrap()
}

#[tauri::command]
pub fn add_to_ln_lib(new_item: LibraryItem) {
  // todo: fix unwraps
  println!("Adding to light novel library...");
  let mut lib = LIB.lock().unwrap();
  if !lib.iter().any(|l| l.id.eq(&new_item.id)) {
    lib.push(new_item);
    save(&lib);
  }
}

#[tauri::command]
pub fn update_ln_lib(item: LibraryItem) {
  let mut lib = LIB.lock().unwrap();
  for entry in lib.iter_mut() {
    if entry.id == item.id {
        *entry = item;
        save(&lib);
        return;
    }
  }
  // If it somehow isn't on the list, add it
  lib.push(item);
}

#[tauri::command]
pub fn remove_from_ln_lib(id: String) {
  // todo: fix unwraps
  println!("Removing from library...");
  let mut lib = LIB.lock().unwrap();
  lib.retain(|l| l.id != id);
  save(&lib);
}

#[tauri::command]
pub fn delete_ln_lib() {
    println!("Deleting ln lib...");
    let mut lib = LIB.lock().unwrap();
    *lib = vec![];
    std::fs::remove_file(&*LIB_PATH).unwrap();
}

#[allow(dead_code)]
pub fn find_ln(id: String) -> Option<LibraryItem> {
  let lib = LIB.lock().unwrap();
  let found_item = lib.iter().find(|item| item.id == id);
  found_item.cloned()
}
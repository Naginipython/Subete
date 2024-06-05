use std::{fs::File, io::Write, sync::Mutex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LIB: Mutex<Vec<LibraryItem>> = match File::open("library.json") {
        Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
        Err(e) => {
          eprintln!("ERROR: {e}");
          Mutex::new(Vec::new())
        }
    };
}


#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryItem {
  id: String,
  title: String,
  img: String,
  extention: String,
  authors: String,
  artists: String,
  description: Option<String>,
  chapters: Vec<ChapterItem>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterItem {
  id: String,
  number: f64,
  title: String,
  page: u32,
  completed: bool
}

fn save(lib: &Vec<LibraryItem>) {
  let mut file = File::create("library.json").unwrap();
  let json = serde_json::to_string(&*lib).unwrap();
  file.write_all(json.as_bytes()).unwrap();
}

#[tauri::command]
pub fn get_lib() -> Value {
  // todo: fix unwraps
  println!("Getting library...");
  let lib = LIB.lock().unwrap();
  serde_json::to_value(&*lib).unwrap()
}

#[tauri::command]
pub fn add_to_lib(new_item: LibraryItem) {
  // todo: fix unwraps
  println!("Adding to library...");
  let mut lib = LIB.lock().unwrap();
  lib.push(new_item);
  save(&*lib);
}

#[tauri::command]
pub fn update_lib(item: LibraryItem) {
  let mut lib = LIB.lock().unwrap();
  for entry in lib.iter_mut() {
    if entry.id == item.id {
        *entry = item;
        save(&*lib);
        return;
    }
  }
  // If it somehow isn't on the list, add it
  lib.push(item);
}

#[tauri::command]
pub fn remove_from_lib(id: String) {
  // todo: fix unwraps
  println!("Removing from library...");
  let mut lib = LIB.lock().unwrap();
  lib.retain(|l| l.id != id);
  save(&*lib);
}
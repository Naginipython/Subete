use crate::{add_to_lib, delete_entire_lib, get_lib, remove_from_lib, update_lib, HasId, FILE_PATH};
use std::{fs::File, path::PathBuf, sync::{LazyLock, Mutex}};
use serde::{Deserialize, Serialize};
use serde_json::Value;

static LIB_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
  let mut path = (*FILE_PATH).clone();
  path.push("ln_library.json");
  path
});
static LIB: LazyLock<Mutex<Vec<LibraryItem>>> = LazyLock::new(|| match File::open(&*LIB_PATH) {
  Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
  Err(_e) => {
      File::create(&*LIB_PATH).unwrap();
      Mutex::new(Vec::new())
  }
});


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
impl PartialEq for LibraryItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.plugin == other.plugin
    }
}
impl HasId for LibraryItem {
    fn id(&self) -> &str {
        &self.id
    }
}

#[tauri::command]
pub fn get_ln_lib() -> Value {
    get_lib("ln", &LIB)
}

#[tauri::command]
pub fn add_to_ln_lib(new_item: LibraryItem) {
    add_to_lib("ln", &LIB, &LIB_PATH, new_item)
}

#[tauri::command]
pub fn update_ln_lib(item: LibraryItem) {
    update_lib("ln", &LIB, &LIB_PATH, item)
}

#[tauri::command]
pub fn remove_from_ln_lib(id: String) {
    remove_from_lib("ln", &LIB, &LIB_PATH, id)
}

#[tauri::command]
pub fn delete_ln_lib() {
    delete_entire_lib("ln", &LIB, &LIB_PATH);
}

#[allow(dead_code)]
pub fn find_ln(id: String) -> Option<LibraryItem> {
  let lib = LIB.lock().unwrap();
  let found_item = lib.iter().find(|item| item.id == id);
  found_item.cloned()
}
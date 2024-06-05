use std::{fs::File, io::Write, sync::Mutex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use lazy_static::lazy_static;

use crate::database::{create_manga, find_manga, get_library, update_manga, Chapter, Manga, NewManga};

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
  pub id: String,
  pub title: String,
  pub img: String,
  pub extension: String,
  pub authors: String,
  pub artists: String,
  pub description: Option<String>,
  pub chapters: Vec<ChapterItem>
}
impl LibraryItem {
  pub fn to_new_manga(&self) -> (NewManga, &Vec<ChapterItem>) {
    let desc = match &self.description {
      Some(desc) => desc,
      None => ""
    };
    (NewManga {
        id: &self.id,
        title: &self.title,
        img: &self.img,
        extension: &self.extension,
        authors: &self.authors,
        artists: &self.artists,
        description: &desc,
    }, &self.chapters)
  }
  pub fn to_manga(self) -> (Manga, Vec<Chapter>) {
    let desc = match self.description {
      Some(desc) => desc,
      None => "".to_string()
    };
    let id = self.id.clone();
    let manga = Manga {
        id: self.id,
        title: self.title,
        img: self.img,
        extension: self.extension,
        authors: self.authors,
        artists: self.artists,
        description: desc,
    };
    let chapters = self.chapters.iter().map(|c| Chapter {
      id: c.id.clone(),
      manga_id: id.clone(),
      number: c.number,
      title: c.title.clone(),
      page: c.page,
      completed: c.completed,
    }).collect();

    (manga, chapters)
  }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterItem {
  pub id: String,
  pub number: f32,
  pub title: String,
  pub page: i32,
  pub completed: bool
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

  // let lib = LIB.lock().unwrap();
  // serde_json::to_value(&*lib).unwrap()
  let lib = get_library().unwrap_or_default();
  serde_json::to_value(lib).unwrap()
}

#[tauri::command]
pub fn add_to_lib(new_item: LibraryItem) {
  // todo: fix unwraps
  println!("Adding to library...");
  // let mut lib = LIB.lock().unwrap();
  // lib.push(new_item);
  // save(&*lib);
  if let Err(e) = create_manga(new_item) {
    eprint!("Error adding to library: {e}")
  }
}

#[tauri::command]
pub fn update_lib(item: LibraryItem) {
  if let Ok(manga) = find_manga(item.id) {
    if let Err(_e) = update_manga(manga) {
      eprintln!("ERROR: Couldn't update manga");
    }
  } else {
    eprintln!("ERROR: Couldn't find manga");
  }
  
  // let mut lib = LIB.lock().unwrap();
  // for entry in lib.iter_mut() {
  //   if entry.id == item.id {
  //       *entry = item;
  //       save(&*lib);
  //       return;
  //   }
  // }
  // // If it somehow isn't on the list, add it
  // lib.push(item);
}

#[tauri::command]
pub fn remove_from_lib(id: String) {
  // todo: fix unwraps
  println!("Removing from library...");
  let mut lib = LIB.lock().unwrap();
  lib.retain(|l| l.id != id);
  save(&*lib);
}
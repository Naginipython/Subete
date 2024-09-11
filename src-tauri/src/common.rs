use std::{fs::File, io::Write, path::PathBuf, sync::Mutex};

use serde::Serialize;
use serde_json::Value;

pub trait HasId {
    fn id(&self) -> &str;
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
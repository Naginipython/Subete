use super::FILE_PATH;
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::{fs::File, io::Write, sync::Mutex};

lazy_static! {
    pub static ref SETTINGS_PATH: String = {
        let mut path = (*FILE_PATH).clone();
        path.push_str("/settings.json");
        path
    };
    pub static ref SETTINGS: Mutex<Value> = match File::open(&*SETTINGS_PATH) {
        Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
        Err(_e) => {
            save(&json!({}));
            Mutex::new(json!({}))
        }
    };
}

fn save(settings: &Value) {
    let mut file = File::create(&*SETTINGS_PATH).unwrap();
    let json = serde_json::to_string(settings).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

#[tauri::command]
pub fn update_settings(new_settings: Value) {
    println!("Updating settings...");
    let mut settings = SETTINGS.lock().unwrap();
    if let Some(setting) = new_settings.as_object() {
        for (key, val) in setting {
            match (*settings).get_mut(key) {
                Some(s) => *s = val.clone(),
                None => {
                    if let Value::Object(ref mut map) = *settings {
                        map.insert(key.to_string(), val.clone());
                    }
                }
            }
        }
    }
    save(&*settings);
}

#[tauri::command]
pub fn get_settings() -> Value {
    // todo: fix unwraps
    println!("Getting Settings...");
    let settings = SETTINGS.lock().unwrap();
    serde_json::to_value(&*settings).unwrap()
}

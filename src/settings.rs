use std::{fs::File, io::Write, sync::Mutex};
use serde_json::{json, Value};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SETTINGS: Mutex<Value> = match File::open("settings.json") {
        Ok(file) => Mutex::new(serde_json::from_reader(file).unwrap_or_default()),
        Err(_e) => {
          save(&json!({}));
          Mutex::new(json!({}))
        }
    };
}

fn save(settings: &Value) {
  let mut file = File::create("settings.json").unwrap();
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
                None => if let Value::Object(ref mut map) = *settings {
                        map.insert(key.to_string(), val.clone());
                    }
            }
        }
    }
    save(&*settings);


//   for entry in lib.iter_mut() {
//     if entry.id == item.id {
//         *entry = item;
//         save(&*lib);
//         return;
//     }
//   }
}

#[tauri::command]
pub fn get_settings() -> Value {
  // todo: fix unwraps
  println!("Getting library...");
  let settings = SETTINGS.lock().unwrap();
  serde_json::to_value(&*settings).unwrap()
}
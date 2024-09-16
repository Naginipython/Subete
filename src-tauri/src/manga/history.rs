use crate::{save, FILE_PATH};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::VecDeque, fs::File, path::PathBuf, sync::LazyLock};

static HIST_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = (*FILE_PATH).clone();
    path.push("manga_history.json");
    path
});

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HistoryItem {
    pub id: String,
    pub title: String,
    pub img: String,
    pub plugin: String,
    pub recent_chapter_id: String,
    pub recent_chapter_num: i32,
    pub timestamp: i64,
}

fn get_hist() -> Vec<HistoryItem> {
    match File::open(&*HIST_PATH) {
        Ok(file) => serde_json::from_reader(file).unwrap_or_default(),
        Err(_e) => {
            File::create(&*HIST_PATH).unwrap();
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn save_manga_history(item: HistoryItem) {
    let temp: Vec<HistoryItem> = get_hist()
        .into_iter()
        .filter(|h| !(h.title == item.title && h.plugin == item.plugin))
        .collect();
    let mut hist = VecDeque::from(temp);

    hist.push_front(item);
    println!("Saving manga history...");
    save(&*HIST_PATH, &Vec::from(hist));
}

#[tauri::command]
pub fn get_manga_history() -> Value {
    // todo: fix unwraps
    println!("Getting manga history...");
    let history = get_hist();
    serde_json::to_value(history).unwrap()
}

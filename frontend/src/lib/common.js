// Commonly used functions
import { invoke } from "@tauri-apps/api/core";
import store from "./store.js";
let json;
store.subscribe(_json => {
    json = _json;
})

export function find_manga(id) {
    let manga;
    let manga_test = json.manga_library.find(m => m.id == id);
    if (manga_test == undefined) {
        let search_test = json.search_results.find(m => m.id == id);
        if (search_test == undefined) {
            manga = json.history.find(m => m.id == id);
        } else {
            manga = search_test;
        }
    } else {
        manga = manga_test;
    }
    return manga;
}
export function in_lib(id) {
    return json.manga_library.some(m => m.id == id);
}

export async function toggle_favorite(manga) {
    console.log(manga);
    if (!in_lib(manga.id)) {
        await invoke('add_to_manga_lib', { newItem: manga });
        store.update(_json => {
            _json.manga_library.push(manga);
            return _json;
        });
    } else {
        await invoke('remove_from_manga_lib', { id: manga.id });
        store.update(_json => {
            _json.manga_library = _json.manga_library.filter(m => m.id != manga.id);
            return _json;
        });
    }
}
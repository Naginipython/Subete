// Commonly used functions
import { invoke } from "@tauri-apps/api/core";
import store from "./store.js";
let json;
store.subscribe(_json => {
    json = _json;
})

export function find_ln(id) {
    let ln;
    let ln_test = json.ln_library.find(m => m.id == id);
    if (ln_test == undefined) {
        let search_test = json.ln_search_results.find(m => m.id == id);
        if (search_test == undefined) {
            ln = json.ln_history.find(m => m.id == id);
        } else {
            ln = search_test;
        }
    } else {
        ln = ln_test;
    }
    return ln;
}
export function in_lib(id) {
    return json.ln_library.some(m => m.id == id);
}

export async function toggle_favorite(ln) {
    if (!in_lib(ln.id)) {
        await invoke('add_to_ln_lib', { newItem: ln });
        store.update(_json => {
            _json.ln_library.push(ln);
            return _json;
        });
    } else {
        await invoke('remove_from_ln_lib', { id: ln.id });
        store.update(_json => {
            _json.ln_library = _json.ln_library.filter(m => m.id != ln.id);
            return _json;
        });
    }
}
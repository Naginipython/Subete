import { invoke } from "@tauri-apps/api/core";
import store from "./store.js";
let json;
store.subscribe(_json => {
    json = _json;
})

export function find_manga(plugin, id) {
    let manga = {};
    manga = json.manga_temp.find(m => m.id==id && (m.plugin==plugin || plugin==null));
    if (manga == undefined) {
        manga = json.manga_library.find(m => m.id == id && (m.plugin==plugin || plugin==null));
        if (manga==undefined) {
            manga = json.manga_search_results.map(m => m.data).flat().find(m => m.id == id && (m.plugin==plugin || plugin==null));
            if (manga==undefined) {
                // hist to manga
                let hist = json.manga_history.find(m => m.id==id && (m.plugin==plugin || plugin==null));
                manga = {
                    id: hist.id,
                    title: hist.title,
                    img: hist.img,
                    plugin: hist.plugin,
                    authors: "",
                    artists: "",
                    description: "",
                    chapters: []
                };
            }
        }
        // add to temp for quicker access
        if (!json.manga_temp.some(m => m.id==id && m.plugin==plugin)) {
            json.manga_temp.unshift(manga);
        }
    }
    return manga==undefined? {} : manga;
}
export function in_lib(id) {
    return json.manga_library.some(m => m.id == id);
}

export async function toggle_favorite(manga) {
    if (!in_lib(manga.id)) {
        await invoke('add_to_manga_lib', { newItem: manga });
        store.update(_json => {
            _json.manga_library.push(manga);
            return _json;
        });
    } else {
        await invoke('remove_from_manga_lib', { id: manga.id });
        store.update(_json => {
            _json.manga_library = _json.manga_library.filter(m => !(m.id==manga.id && m.plugin==manga.plugin));
            return _json;
        });
    }
}

export async function update_lib() {
    // re-fetch every manga chapter, comapre to what we have, add to manga_updates
    const updated = () => store.update(_json => { _json.manga_updates_progress = json.manga_update_progress; return _json; });
    json.manga_update_progress = {
        curr: "",
        index: 0,
        total: json.manga_library.length
    };
    updated();

    for (let i = 0; i < json.manga_library.length; i++) {
        // Progress bar
        json.manga_update_progress.curr = json.manga_library[i].title;
        json.manga_update_progress.index = json.manga_update_progress.index+1;
        updated();

        // Update
        let new_data = await invoke('get_manga_chapters', { manga: json.manga_library[i] });
        let chap = new_data.chapters;

        // Gets new chapters
        let new_chapters = chap.filter(c => {
            return !json.manga_library[i].chapters.some(o => o.id == c.id);
        }).reverse();
        json.manga_library[i] = new_data;

        // Adds to updates list
        for (let j = 0; j < new_chapters.length; j++) {
            let item = {
                id: json.manga_library[i].id,
                title: json.manga_library[i].title,
                img: json.manga_library[i].img,
                plugin: json.manga_library[i].plugin,
                chapter: new_chapters[j],
                received: Date.now()
            }
            store.update(_json => {
                _json.manga_updates.unshift(item);
                _json.manga_library = json.manga_library;
                return _json;
            });
        }
        await invoke('update_manga_lib', { item: json.manga_library[i] })
    }
    await invoke('save_manga_updates_list', { items: json.manga_updates });
    json.manga_update_progress = null;
    updated();
}

export function find_chapter_index_by_id(id, chap_id) {
    let manga = find_manga(null, id);
    return manga.chapters.findIndex(c => c.id == chap_id);
}
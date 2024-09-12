import { invoke } from "@tauri-apps/api/core";
import store from "./store.js";
let json;
store.subscribe(_json => {
    json = _json;
});

export function find_anime(plugin, id) {
    let anime = {};
    anime = json.anime_temp.find(m => m.id==id && (m.plugin==plugin || plugin==null));
    if (anime == undefined) {
        anime = json.anime_library.find(m => m.id == id && (m.plugin==plugin || plugin==null));
        if (anime==undefined) {
            anime = json.anime_search_results.find(m => m.id == id && (m.plugin==plugin || plugin==null));
            if (anime==undefined) {
                // hist to anime
                let hist = json.anime_history.find(m => m.id==id && (m.plugin==plugin || plugin==null));
                anime = {
                    id: hist.id,
                    title: hist.title,
                    img: hist.img,
                    plugin: hist.plugin,
                    studio: "",
                    status: "",
                    description: "",
                    episodes: []
                };
            }
        }
        // add to temp for quicker access
        if (!json.anime_temp.some(m => m.id==id && m.plugin==plugin)) {
            json.anime_temp.unshift(anime);
        }
    }
    return anime==undefined? {} : anime;
}
export function in_lib(id) {
    return json.anime_library.some(m => m.id == id);
}

export async function toggle_favorite(anime) {
    if (!in_lib(anime.id)) {
        await invoke('add_to_anime_lib', { newItem: anime });
        store.update(_json => {
            _json.anime_library.push(anime);
            return _json;
        });
    } else {
        await invoke('remove_from_anime_lib', { id: anime.id });
        store.update(_json => {
            _json.anime_library = _json.anime_library.filter(m => !(m.id==anime.id && m.plugin==anime.plugin));
            return _json;
        });
    }
}

export async function update_lib() {
    // re-fetch every anime chapter, comapre to what we have, add to anime_updates
    // const updated = () => store.update(_json => { _json.anime_updates_progress = json.anime_update_progress; return _json; });
    // json.anime_update_progress = {
    //     curr: "",
    //     index: 0,
    //     total: json.anime_library.length
    // };
    // updated();

    // for (let i = 0; i < json.anime_library.length; i++) {
    //     // Progress bar
    //     json.anime_update_progress.curr = json.anime_library[i].title;
    //     json.anime_update_progress.index = json.anime_update_progress.index+1;
    //     updated();

    //     // Update
    //     let new_data = await invoke('get_anime_chapters', { anime: json.anime_library[i] });
    //     let chap = new_data.chapters;

    //     // Gets new chapters
    //     let new_chapters = chap.filter(c => {
    //         return !json.anime_library[i].chapters.some(o => o.id == c.id);
    //     }).reverse();
    //     json.anime_library[i] = new_data;

    //     // Adds to updates list
    //     for (let j = 0; j < new_chapters.length; j++) {
    //         let item = {
    //             id: json.anime_library[i].id,
    //             title: json.anime_library[i].title,
    //             img: json.anime_library[i].img,
    //             plugin: json.anime_library[i].plugin,
    //             chapter: new_chapters[j],
    //             received: Date.now()
    //         }
    //         store.update(_json => {
    //             _json.anime_updates.unshift(item);
    //             _json.anime_library = json.anime_library;
    //             return _json;
    //         });
    //     }
    //     await invoke('update_anime_lib', { item: json.anime_library[i] })
    // }
    // await invoke('save_anime_updates_list', { items: json.anime_updates });
    // json.anime_update_progress = null;
    // updated();
}

export function find_episode_index_by_id(id, ep_id) {
    let anime = find_anime(null, id);
    return anime.episodes.findIndex(c => c.id == ep_id);
}
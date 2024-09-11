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
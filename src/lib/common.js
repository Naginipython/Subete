import { invoke } from "@tauri-apps/api/core";
import store from "$lib/store.js";
let json;
store.subscribe(_json => {
    json = _json;
})

export async function setup(media_screen) {
    let manga_library = await invoke('get_manga_lib');
    let manga_history = await invoke('get_manga_history');
    let manga_updates = await invoke('get_manga_updates_list');
    let anime_library = await invoke('get_anime_lib');
    let anime_updates = await invoke('get_anime_updates_list');
    let ln_library = await invoke('get_ln_lib');
    let ln_updates = await invoke('get_ln_updates_list');
    let settings = await invoke('get_settings');
    store.update(json => {
        json.manga_library = manga_library;
        json.manga_history = manga_history;
        json.manga_updates = manga_updates;
        json.anime_library = anime_library;
        json.anime_updates = anime_updates;
        json.ln_library = ln_library;
        json.ln_updates = ln_updates;
        json.settings = settings;
        json.media_screen = media_screen;
        return json;
    });

    // SETS AND INITS SETTINGS
    if (!settings.hasOwnProperty("app_colors")) {
        // init
        settings['app_colors'] = {theme: "Nagini's dark", primary: "1a1a1a", secondary: "330000", selection: "800000", text: "ffffff"};
    }
    document.documentElement.style.setProperty('--primary-color', `#${settings['app_colors'].primary}`); 
    document.documentElement.style.setProperty('--secondary-color', `#${settings['app_colors'].secondary}`); 
    document.documentElement.style.setProperty('--selection-color', `#${settings['app_colors'].selection}`); 
    document.documentElement.style.setProperty('--text-color', `#${settings['app_colors'].text}`); 

    if (!settings.hasOwnProperty("library_widths")) {
        // init
        settings['library_widths'] = { manga: '100', ln: '100', anime: '100' };

    }
    document.documentElement.style.setProperty('--lib-manga-width', `${settings['library_widths'].manga}px`); 
    document.documentElement.style.setProperty('--lib-ln-width', `${settings['library_widths'].ln}px`); 
    document.documentElement.style.setProperty('--lib-anime-width', `${settings['library_widths'].ln}px`); 
    
    // sets inits
    store.update(json => {
        json.settings = settings;
        return json;
    });
}

export function find_item(type, plugin, id) {
    let item = {};
    item = json[`${type}_temp`].find(i => i.id==id && (i.plugin==plugin || plugin==null));
    if (item == undefined) {
        item = json[`${type}_library`].find(i => i.id == id && (i.plugin==plugin || plugin==null));
        if (item==undefined) {
            item = json[`${type}_search_results`].map(i => i.data).flat().find(i => i.id == id && (i.plugin==plugin || plugin==null));
            if (item==undefined) {
                // hist to item
                let hist = json[`${type}_history`].find(i => i.id==id && (i.plugin==plugin || plugin==null));
                if (type=="anime") {
                    item = {
                        id: hist.id,
                        title: hist.title,
                        img: hist.img,
                        plugin: hist.plugin,
                        studio: "",
                        status: "",
                        description: "",
                        episodes: []
                    }
                } else {
                    item = {
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
        }
        // add to temp for quicker access
        if (!json[`${type}_temp`].some(i => i.id==id && i.plugin==plugin)) {
            json[`${type}_temp`].unshift(item);
        }
    }
    return item==undefined? {} : item;
}
export function in_lib(type, id) {
    return json[`${type}_library`].some(i => i.id == id);
}
export async function toggle_favorite(type, item) {
    if (!in_lib(type, item.id)) {
        await invoke(`add_to_${type}_lib`, { newItem: item });
        store.update(_json => {
            _json[`${type}_library`].push(item);
            return _json;
        });
    } else {
        await invoke(`remove_from_${type}_lib`, { id: item.id });
        store.update(_json => {
            _json[`${type}_library`] = _json[`${type}_library`].filter(i => !(i.id==item.id && i.plugin==item.plugin));
            return _json;
        });
    }
}
export async function update(type) {
    // vars for easier readability
    let entry_type = type=="anime"? "episodes" : "chapters";
    let lib = `${type}_library`;
    let progress = `${type}_update_progress`;
    
    const updated = () => store.update(_json => { _json[progress] = json[progress]; return _json; });
    json[progress] = {
        curr: "",
        index: 0,
        total: json[lib].length
    };
    updated();

    // re-fetch every item entry, comapre to what we have, add to updates
    for (let i = 0; i < json[lib].length; i++) {
        // Progress bar
        json[progress].curr = json[lib][i].title;
        json[progress].index = json[progress].index+1;
        updated();

        // Update
        let new_data = await invoke(`get_${type}_${entry_type}`, { item: json[lib][i] });
        let entry = new_data[entry_type];

        // Gets new entries
        let new_entries = entry.filter(e => {
            return !json[lib][i][entry_type].some(o => o.id == e.id);
        }).reverse();
        json[lib][i] = new_data;

        // Adds to updates list
        for (let j = 0; j < new_entries.length; j++) {
            let item = type=="anime"? {
                    id: json[lib][i].id,
                    title: json[lib][i].title,
                    img: json[lib][i].img,
                    plugin: json[lib][i].plugin,
                    episode: new_entries[j],
                    received: Date.now()
                } : {
                    id: json[lib][i].id,
                    title: json[lib][i].title,
                    img: json[lib][i].img,
                    plugin: json[lib][i].plugin,
                    chapter: new_entries[j],
                    received: Date.now()
                };
            store.update(_json => {
                // removes from [item]_temp
                let dropIndex = _json[`${type}_temp`].findIndex(t => t.id == json[lib][i].id && t.plugin == json[lib][i].plugin);
                _json[`${type}_temp`].splice(dropIndex, 1);
                // item = json[`${type}_temp`].find(i => i.id==id && (i.plugin==plugin || plugin==null));

                _json[`${type}_updates`].unshift(item);
                _json[lib] = json[lib];
                return _json;
            });
        }
        await invoke(`update_${type}_lib`, { item: json[lib][i] })
    }
    await invoke(`save_${type}_updates_list`, { items: json[`${type}_updates`] });
    json[progress] = null;
    updated();
}
export function find_entry_index_by_id(type, id, entry_id) {
    let item = find_item(type, null, id);
    if (type == "anime") {
        return item.episodes.findIndex(c => c.id == entry_id);
    } else {
        return item.chapters.findIndex(c => c.id == entry_id);
    }
}
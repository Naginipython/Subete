import { invoke } from "@tauri-apps/api/core";
import store from "$lib/store.js";

export async function setup(media_screen) {
    let manga_library = await invoke('get_manga_lib');
    let manga_history = await invoke('get_manga_history');
    let manga_updates = await invoke('get_manga_updates_list');
    let anime_library = await invoke('get_anime_lib');
    let ln_library = await invoke('get_ln_lib');
    let settings = await invoke('get_settings');
    store.update(json => {
        json.manga_library = manga_library;
        json.manga_history = manga_history;
        json.manga_updates = manga_updates
        json.ln_library = ln_library;
        json.anime_library = anime_library;
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
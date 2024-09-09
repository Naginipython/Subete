import { writable } from 'svelte/store';
// Note: Ideally, this is the overall structure of the store
const store = writable({
    "media_screen": "manga",
    "manga_search_results": [
        /*{
            id: int
            title: string,
            img: string, //url or link?
            plugin: string, //file inside lib/manga_sources without "./" and ".js". e.g. "mangadex"
            authors: string,
            artists: string
            description: string,
            chapters: [
                {
                    id: string,
                    number: int,
                    title: string,
                    page: int,
                    completed: boolean
                }
            ]
        }*/
    ],
    "manga_library": [
        /*{
            id: string
            title: string,
            img: string, //url or link?
            plugin: string, //file inside lib/manga_sources without "./" and ".js". e.g. "mangadex"
            authors: string,
            artists: string
            description: string,
            chapters: [
                {
                    id: string,
                    number: int,
                    title: string,
                    page: int,
                    completed: boolean
                }
            ]
            // todo:
            // language?
        }*/
    ],
    "manga_temp": [
        //TODO: Will hold every manga_lib entry pressed, from lib and search. To be wiped upon app close. Solves fav issue, and cleans up manga_common
    ],
    "manga_history": [
        /*{
            id: String,
            title: String,
            img: String,
            plugin: String,
            recent_chapter_id: String,
            recent_chapter_num: int,
            timestamp: double
        }*/
    ],
    "manga_updates": [
        /*{
            id: string,
            title: String,
            img: String, //url or link?
            plugin: String,
            chapter: {
                id: string,
                number: int,
                title: string,
                page: int,
                completed: boolean
            }
            timestamp: double // Date.now()
        }*/
    ],
    "manga_update_progress": null, // {curr: String, index: number, total: number}
    "ln_search_results": [],
    "ln_library": [],
    "ln_history": [],
    "ln_updates": [],
    "settings": {
        "quickselect": {}, // typically: "[plugin_name]": Boolean, 
        "library_widths": {}, // "manga", "ln", and "anime", equal to a string of the width #
    }
});
export default store;

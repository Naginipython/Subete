import { writable } from 'svelte/store';
// Note: Ideally, this is the overall structure of the store
const store = writable({
    "media_screen": "manga",
    "search_results": [
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
                    num: int,
                    title: string,
                    page: int,
                    completed: boolean
                }
            ]
        }*/
    ],
    "manga_library": [
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
                    num: int,
                    title: string,
                    page: int,
                    completed: boolean
                }
            ]
            // todo:
            // language?
        }*/
    ],
    "history": [
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
                    num: int,
                    title: string,
                    page: int,
                    completed: boolean
                }
            ]
        }*/
    ],
    "settings": {
        "quickselect": {}, // typically: "[plugin_name]": Boolean, 
    }
});
export default store;

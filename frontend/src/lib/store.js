import { writable } from 'svelte/store';
// Note: Ideally, this is the overall structure of the store
const store = writable({
    "manga_return": "/",
    "search_results": [
        /*{
            id: int
            title: string,
            img: string, //url or link?
            extention: string, //file inside lib/manga_sources without "./" and ".js". e.g. "mangadex"
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
    "library": [
        /*{
            id: int
            title: string,
            img: string, //url or link?
            extention: string, //file inside lib/manga_sources without "./" and ".js". e.g. "mangadex"
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
            extention: string, //file inside lib/manga_sources without "./" and ".js". e.g. "mangadex"
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
    ]
});
export default store;

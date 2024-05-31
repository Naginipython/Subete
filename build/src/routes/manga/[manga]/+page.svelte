<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { navigating } from '$app/stores';
    import store from "$lib/store.js"
    import { getChapters } from "$lib/manga_sources/main.js";

    export let data;

    let manga = {};
    let favorited = false;

    // Adds to history when data is available
    $: if (Object.keys(manga).length != 0) {
        store.update(json => {
            json.history.push(manga);
            return json;
        });
    }
    
    // sets back button
    let back = "/";
    $: if($navigating) set_back();
    function set_back() {
        switch ($navigating.from.url.pathname) {
            case "/library":
            case "/updates":
            case "/browse":
                store.update(json => {
                    json.manga_return = $navigating.from.url.pathname;
                    back = json.manga_return;
                    return json;
                });
        }
    }

    store.subscribe(async (json) => {
        // gets manga search details
        // todo: this doesnt work because object is search_results[#]['data']
        let manga_test = json.library.find(m => m.id == data.id);
        if (manga_test == undefined) {
            let search_test = json.search_results.find(m => m.id == data.id);
            if (search_test == undefined) {
                manga = json.history.find(m => m.id == data.id);
            } else {
                manga = search_test;
            }
        } else {
            manga = manga_test;
        }

        // gets chapters, if needed
        if (manga['chapters'].length == 0) {
            manga['chapters'] = await getChapters(manga.extention, manga.id);
        }
        console.log(manga['chapters'][0].page);

        // checks if in library
        let lib_item = json.library.find(l => l.id == manga.id);
        if (lib_item != -1 && lib_item != undefined) {
            favorited = true;
        }
        
        // sets back button
        back = json.manga_return;
    });

    async function toggle_fav() {
        favorited = !favorited;
        if (favorited) {
            await invoke('add_to_lib', { newItem: manga });
            store.update(json => {
                json.library.push(manga);
                return json;
            });
        } else {
            await invoke('remove_from_lib', { id: manga.id });
            store.update(json => {
                json.library = json.library.filter(m => m.id != manga.id);
                return json;
            });
        }
    }
</script>

<div id="top-bar">
    <a class="return" href="{back}">&lt;</a>
    <button class="fav-btn" on:click={async () =>  toggle_fav()}>{favorited? '♥' : '♡'}</button><br>
</div>

<div id="header" >
    <div id="img-wrapper">
        <img id="img" src="{manga.img}" alt="{manga.title}"/>
    </div>
    <div id="header-content">
        <div id="text">
            <h3>{manga.title}</h3>
            <p>Author: {manga.authors}</p>
            <p>Extention: {manga.extention}</p>
            <div id="desc"><p>{manga.description}</p></div>
        </div>
    </div>
</div>


{#each manga['chapters'] as c, i}
<div class="chapter">
    <a class="btn" href="/manga/{data.id}/reader/{i}">{c.number}: {c.title}</a>
    {#if manga['chapters'][i].page-1 != 0}
        <p class="progress">&emsp;(page: {manga['chapters'][i].page})</p>
    {/if}
</div><br>
{/each}

<style>
    #top-bar {
        overflow: hidden;
    }
    .fav-btn {
        padding: 0 10px;
        font-size: xx-large;
        float: right;
    }
    #header {
        padding: 10px;
        padding-top: 0px;
        overflow: hidden;
        border: 1px solid black;
    }
    #img-wrapper {
        width: 200px;
        height: 300px;
        border-radius: 10px;
        overflow: hidden;
        justify-content: center;
        align-items: center;
        float: left;
    }
    #img {
        height: 105%;
        width: auto;
    }
    #header-content {
        display: flex;
        justify-content: left;
        float: right;
        overflow: hidden;
        width: calc(100% - 250px);
        padding-right: 10px;
        height: 300px;
    }
    #text {
        height: inherit;
        /* overflow: hidden; */
    }
    #text p {
        margin: 10px;
        font-size: smaller;
    }
    #desc {
        overflow: scroll;
    }
    .return {
        font-size: xx-large;
        padding: 15px;
        width: 20vw;
        text-decoration: none;
        align-items: flex-start;
        text-align: center;
        cursor: default;
        color: white;
        padding-block-start: 2px;
        padding-block-end: 3px;
        border-top-width: 2px;
        border-right-width: 2px;
        border-bottom-width: 2px;
        border-left-width: 2px;
        border-top-style: outset;
        border-right-style: outset;
        border-bottom-style: outset;
        border-left-style: outset;
        border-top-color: buttonface;
        border-right-color: buttonface;
        border-bottom-color: buttonface;
        border-left-color: buttonface;
        box-sizing: border-box;
    }
    .chapter {
        display: inline-flex;
        justify-content: left;
        align-items: center;
    }
    .btn {
        
        background: none;
        color: inherit;
        border: none;
        padding: 0;
        font: inherit;
        cursor: pointer;
        outline: inherit;
    }
    .progress {
        color: grey;
        margin: 0;
        padding: 0;
        font-size: x-small;
    }
</style>
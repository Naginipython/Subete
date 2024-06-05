<script>
    import { faMagnifyingGlass, faXmark } from '@fortawesome/free-solid-svg-icons'
    // import { faHeart as faOutlineHeart } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa'
    import store from "$lib/store.js";
    // import { searchManga, getSources } from "$lib/manga_sources/main.js";
    import DisplayManga from "./display_manga.svelte";
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from "svelte";

    let name = '';
    let results = [];
    let sources = [];
    // todo: save checked_sources to disk
    let checked_source = {"mangadex": true};

    onMount(async () => {
        sources = await invoke('get_plugin_names');
    });

    store.subscribe(json => {
        results = json["search_results"];
        reformatResults();
        return json;
    });

    async function search() {
        results = [];
        let s = [];
        for (let [key, value] of Object.entries(checked_source)) {
            if (value) {
                s.push(key);
            }
        }
        results = await invoke('search', { query: `${name}`, sources: s });
        console.log(results);
        
        store.update(json => {
            json.search_results = results;
            return json;
        });
        
        reformatResults();
    }

    function reformatResults() {
        // Exports the extension name to the outside.
        // Turns a array of everything into:
        // [{extension: string, data: []}]
        if (!results.some(a => a.hasOwnProperty('data'))) {
            results = Object.values(
                results.reduce((result, item) => {
                    let extension = item.extension;
                    if (!result[extension]) {
                        result[extension] = {extension: extension, data: []};
                    }
                    result[extension]['data'].push(item);
                    return result;
                }, {})
            );
        }
    }
    function clear_search() {
        results = [];
        store.update(json => {
            json.search_results = results;
            return json;
        });
    }
</script>

<div>
    <!-- Search box -->
    <form id="form">
        <!-- todo: center content in this properly -->
        <div id="form-input">
            <input id="clear_search" on:click="{clear_search}" style="{results.length == 0? 'display:none':''}" type="reset" value="X">
            <input id="input" placeholder="Enter a title..." bind:value="{name}" style="{results.length == 0? '':'width:90vw'}" />
            <button id="search" on:click="{search}"><Fa icon={faMagnifyingGlass} /></button><br>
        </div>
    </form>
    {#if results.length == 0}
    <div class="quickselect">
        <p>Source Quickselect</p>
        {#each sources as s, i}
        <div>
            <label for="source-{i}">{s}</label>
            <input id="source-{i}" type="checkbox" bind:checked={checked_source[s]}>
        </div>
        {/each}
    </div>
    {/if}

    <!-- displays manga -->
    {#each results as item, i}
        <h3>{item.extension}</h3>
        <DisplayManga data={item.data}/>
    {/each}

    

</div>
<style>
    #form {
        width: 100vw;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    /* Searchbox (TO CHANGE) */
    #form-input {
        display: inline-flex;
        margin-top: 10px;
        width: 98vw;
    }
    #input {
        appearance: none;
        border: 1px solid black;
        width: 94vw;
        background-color: var(--secondary-color);
        color: white;
    }
    #clear_search {
        appearance: none;
        background-color: white;
        text-shadow: none;
        border: 1px solid black;
        padding: 1px 6px;
        border-radius: 5px 0 0 5px;
        margin-left: -5px;
        background-color: var(--secondary-color);
        color: white;
        width: 4vw;
    }
    #search {
        margin: 0;
        appearance: none;
        background-color: white;
        text-shadow: none;
        border: 1px solid black;
        padding: 1px 6px;
        border-radius: 0 5px 5px 0;
        margin-left: -5px;
        background-color: var(--secondary-color);
        color: white;
        width: 4vw;
    }
    .quickselect {
        display: inline-flex;
        justify-content: left;
        flex-direction: column;
        margin: 10px;
    }
</style>
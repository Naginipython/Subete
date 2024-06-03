<script>
    import { faMagnifyingGlass, faXmark } from '@fortawesome/free-solid-svg-icons'
    // import { faHeart as faOutlineHeart } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa'
    import store from "$lib/store.js";
    import { searchManga, getSources } from "$lib/manga_sources/main.js";
    import DisplayManga from "./display_manga.svelte";

    let name = '';
    let results = [];
    let sources = getSources();
    // todo: save checked_sources to disk
    let checked_source = {"mangadex": true};

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
        results = await searchManga(name, s);
        console.log(results);
        
        store.update(json => {
            json.search_results = results;
            return json;
        });
        
        reformatResults();
    }

    function reformatResults() {
        // Exports the extention name to the outside.
        // Turns a array of everything into:
        // [{extention: string, data: []}]
        if (!results.some(a => a.hasOwnProperty('data'))) {
            results = Object.values(
                results.reduce((result, item) => {
                    let extention = item.extention;
                    if (!result[extention]) {
                        result[extention] = {extention: extention, data: []};
                    }
                    result[extention]['data'].push(item);
                    return result;
                }, {})
            );
        }
    }
    function clear_search() {
        results = [];
    }
</script>

<div>
    <!-- Search box -->
    <form id="form">
        <!-- todo: center content in this properly -->
        <div style="display:inline-flex;justify-content:center;margin:auto">
            <button id="clear_search" on:click="{clear_search}" style="{results.length == 0? 'display:none':''}"><Fa icon={faXmark} /></button>
            <input id="input" placeholder="Enter a title..." bind:value="{name}" />
            <button id="search" on:click="{search}"><Fa icon={faMagnifyingGlass} /></button><br>
        </div>
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
    </form>

    <!-- displays manga -->
    {#each results as item, i}
        <h3>{item.extention}</h3>
        <DisplayManga data={item.data}/>
    {/each}

    

</div>
<style>
    /* Searchbox (TO CHANGE) */
    #form {
        margin: 10px;
    }
    #input {
        appearance: none;
        border: 1px solid black;
        width: 95vw;
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
    }
    #search {
        appearance: none;
        background-color: white;
        text-shadow: none;
        border: 1px solid black;
        padding: 1px 6px;
        border-radius: 0 5px 5px 0;
        margin-left: -5px;
        background-color: var(--secondary-color);
        color: white;
    }
    .quickselect {
        display: inline-flex;
        flex-direction: column;

    }
</style>
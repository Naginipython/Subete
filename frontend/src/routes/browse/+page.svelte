<script>
    import { faMagnifyingGlass, faXmark } from '@fortawesome/free-solid-svg-icons'
    // import { faHeart as faOutlineHeart } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa'
    import store from "$lib/store.js";
    import DisplayManga from "./display_manga.svelte";
    import { invoke } from '@tauri-apps/api/tauri';

    let name = '';
    let results = [];
    let sources = [];
    let checked_sources;
    let first_run = true;
    $: if (checked_sources) update_settings();
    
    async function update_settings() {
        // console.log(checked_sources)
        if (!first_run) {
            store.update(json => {
                json["settings"].quickselect = checked_sources;
                return json;
            });
            await invoke('update_settings', { newSettings: {"quickselect":checked_sources}})
        }
        first_run = false;
    }

    store.subscribe(async json => {
        results = json["search_results"];
        if (!json["settings"].hasOwnProperty("quickselect")) {
            checked_sources = {"MangaDex": true};
        } else {
            checked_sources = json["settings"].quickselect;
        }
        sources = await invoke('get_manga_plugin_names');
        let s = Object.entries(checked_sources).map(([key, value]) => key);
        let new_sources = sources.filter(source => !s.includes(source));
        new_sources.forEach(i => {
            checked_sources[i] = true;
        });
        reformatResults();
        return json;
    });

    async function search() {
        results = [];
        let s = Object.entries(checked_sources).filter(([key, value]) => value).map(([key, value]) => key);
        console.log(s);
        results = await invoke('manga_search', { query: `${name}`, sources: s });
        
        store.update(json => {
            json.search_results = results;
            return json;
        });
        
        reformatResults();
    }

    function reformatResults() {
        // Exports the plugin name to the outside.
        // Turns a array of everything into:
        // [{plugin: string, data: []}]
        if (!results.some(a => a.hasOwnProperty('data'))) {
            results = Object.values(
                results.reduce((result, item) => {
                    let plugin = item.plugin;
                    if (!result[plugin]) {
                        result[plugin] = {plugin: plugin, data: []};
                    }
                    result[plugin]['data'].push(item);
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
            <input id="source-{i}" type="checkbox" bind:checked={checked_sources[s]}>
        </div>
        {/each}
    </div>
    {/if}

    <!-- displays manga -->
    {#each results as item, i}
        <h3>{item.plugin}</h3>
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
        color: var(--text-color);
    }
    #clear_search {
        appearance: none;
        background-color: var(--text-color);
        text-shadow: none;
        border: 1px solid black;
        padding: 1px 6px;
        border-radius: 5px 0 0 5px;
        margin-left: -5px;
        background-color: var(--secondary-color);
        color: var(--text-color);
        width: 4vw;
    }
    #search {
        margin: 0;
        appearance: none;
        text-shadow: none;
        border: 1px solid black;
        padding: 1px 6px;
        border-radius: 0 5px 5px 0;
        margin-left: -5px;
        background-color: var(--secondary-color);
        color: var(--text-color);
        width: 4vw;
    }
    .quickselect {
        display: inline-flex;
        justify-content: left;
        flex-direction: column;
        margin: 10px;
    }
</style>
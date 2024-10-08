<script>
    import { faMagnifyingGlass, faXmark } from '@fortawesome/free-solid-svg-icons'
    import Fa from 'svelte-fa'
    import store from "$lib/store.js";
    import Display from "./display.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Moon } from 'svelte-loading-spinners';

    let name = '';
    let results = [];
    let sources = [];
    let checked_sources;
    let media_screen = "manga";
    
    $: if (checked_sources) update_settings();
    let first_run = true;
    async function update_settings() {
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
        media_screen = json["media_screen"];
        if (!json["settings"].hasOwnProperty("quickselect")) {
            checked_sources = {"MangaDex": true};
        } else {
            checked_sources = json["settings"].quickselect;
        }
        switch (media_screen) {
            case "manga":
                results = json["manga_search_results"];
                sources = await invoke('get_manga_plugin_names');
                break;
            case "ln":
                results = json["ln_search_results"];
                sources = await invoke('get_ln_plugin_names');
                break;
            case "anime":
                results = json["anime_search_results"];
                sources = await invoke('get_anime_plugin_names');
                break;
        }
        let s = Object.entries(checked_sources).map(([key, value]) => key);
        let new_sources = sources.filter(source => !s.includes(source));
        new_sources.forEach(i => {
            checked_sources[i] = true;
        });
        return json;
    });

    let is_searching = false;
    async function search() {
        is_searching = true;
        results = [];
        // setup sources to be searched and the output
        let search_sources = Object.entries(checked_sources).filter(([key, value]) => value).map(([key, value]) => key);
        search_sources = search_sources.filter(x => sources.includes(x));
        let result = [];
        for (const s of search_sources) {
            result.push({plugin: s, data: [], error: ""});
        }
        // based on media_screen, calls different tauri commands & stores differently
        switch (media_screen) {
            case "manga":
                for (const s of search_sources) {
                    let r = await invoke('manga_search', { query: `${name}`, source: s });
                    if (!r.hasOwnProperty("error")) {
                        result[result.findIndex(i => i.plugin==s)].data = r;
                    } else {
                        result[result.findIndex(i => i.plugin==s)].error = r.error;
                    }
                    store.update(json => {
                        json.manga_search_results = result;
                        return json;
                    });
                }
                break;
            case "ln":
                for (const s of search_sources) {
                    let r = await invoke('ln_search', { query: `${name}`, source: s });
                    if (!r.hasOwnProperty("error")) {
                        result[result.findIndex(i => i.plugin==s)].data = r;
                    } else {
                        result[result.findIndex(i => i.plugin==s)].error = r.error;
                    }
                    store.update(json => {
                        json.ln_search_results = result;
                        return json;
                    });
                }
                break;
            case "anime":
                for (const s of search_sources) {
                    let r = await invoke('anime_search', { query: `${name}`, source: s });
                    if (!r.hasOwnProperty("error")) {
                        result[result.findIndex(i => i.plugin==s)].data = r;
                    } else {
                        result[result.findIndex(i => i.plugin==s)].error = r.error;
                    }
                    store.update(json => {
                        json.anime_search_results = result;
                        return json;
                    });
                }
                break;
        }
        is_searching = false;
    }

    function clear_search() {
        results = [];
        store.update(json => {
            switch (media_screen) {
                case "manga":
                    json.manga_search_results = results;
                    break;
                case "ln":
                    json.ln_search_results = results;
                    break;
                case "anime":
                    json.anime_search_results = results;
                    break;
            }
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
            <input id="input" placeholder="Enter a title..." bind:value="{name}" style="{results.length == 0? '':'width:80%'}" />
            <button id="search" on:click="{search}"><Fa icon={faMagnifyingGlass} /></button><br>
        </div>
    </form>
    {#if results.length == 0 && !is_searching}
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

    {#if results.length==0 && is_searching}
        <div class="loading">
            <Moon color="var(--selection-color)" size="30" />
        </div>
    {/if}

    <!-- displays manga -->
    {#each results as item, i}
        <h3>{item.plugin}</h3>
        {#if item.data.length==0 && is_searching}
            <div class="loading">
                <Moon color="var(--selection-color)" size="30" />
            </div>
        {:else}
            <Display data={item.data} media_screen={media_screen} error={item.error}/>
        {/if}
    {/each}

</div>
<style>
    #form {
        width: 98vw;
        margin: auto;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    /* Searchbox (TO CHANGE) */
    #form-input {
        display: inline-flex;
        height: 35px;
        margin-top: 10px;
        /* width: 98vw; */
        width: 100%;
    }
    #input {
        appearance: none;
        border: 1px solid black;
        /* width: 94vw; */
        width: 90%;
        background-color: var(--secondary-color);
        color: var(--text-color);
        font-size: large;
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
        width: 10%;
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
        /* width: 4vw; */
        width: 10%;
    }
    .quickselect {
        display: inline-flex;
        justify-content: left;
        flex-direction: column;
        margin: 10px;
    }
    .loading {
        padding: 20px;
        margin: auto;
        width: fit-content;
    }
</style>
<script>
    import store from "$lib/store.js";
    import DisplayManga from "./display_manga.svelte";
    import { searchManga } from "$lib/manga_sources/main.js";

    let name = '';
    let results = [];

    store.subscribe(json => {
        results = json["search_results"];
        reformatResults();
        return json;
    });

    async function search() {
        results = [];
        results = await searchManga(name);
        
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
</script>

<div>
    <!-- Search box -->
    <form id="form">
        <input id="input" placeholder="Enter a title..." bind:value="{name}" />
        <button id="search" on:click="{search}">&gt;</button><br>
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
        display: flex; 
        justify-content: center
    }
    #input {
        appearance: none;
        border: 1px solid black;
        width: 95vw;
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
</style>
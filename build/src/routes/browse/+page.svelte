<script>
    import store from "$lib/store.js";
    import DisplayManga from "./display_manga.svelte";
    import { searchManga } from "$lib/manga_sources/main.js";

    let name = '';
    let data = [];

    store.subscribe(json => {
        data = json["search_results"];
        return json;
    });

    async function search() {
        data = [];

        let temp = await searchManga(name);
        data = temp[0].data;
        
        store.update(json => {
            json.search_results = data;
            return json;
        });
        data = data;
    }
</script>

<div>
    <!-- Search box -->
    <form style="display:flex; justify-content:center">
        <input id="input" placeholder="Enter a title..." bind:value="{name}" />
        <button id="search" on:click="{search}">&gt;</button><br>
    </form>

    <!-- displays manga -->
    <DisplayManga {data}/>

</div>
<style>
    :global(body) {
        margin: 0;
    }
    /* Searchbox (TO CHANGE) */
    #input {
        appearance: none;
        border: 1px solid black;
        width: 95vw;
    }
    #search {
        appearance: none;
        background-color: white;
        text-shadow: none;
        border: 1px solid black;
        padding: 1px 6px;
        border-radius: 0 5px 5px 0;
        margin-left: -5px;
    }
</style>
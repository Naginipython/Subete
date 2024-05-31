<style>
    :root {
        --snackbar-height: 25px;
        --nav-bar-height: 50px;
        --primary-color: #1a1a1a;
        --secondary-color: #330000;
        --selection-color: maroon;
    }
    #snackbar {
        height: var(--snackbar-height);
        background-color: var(--secondary-color);
    }
    .snackbar-item {
        display: inline-flex;
        align-items: center;
        background-color: var(--secondary-color);
        /* background-color: green; */
        border: 0;
        color: white;
        font-size: medium;
        padding: 0 10px;
        height: inherit;
    }
    .snackbar-item:hover {
        background-color: var(--selection-color);
    }
    #body {
        height: calc(100vh - var(--snackbar-height) - var(--nav-bar-height));
        color: white;
        overflow: scroll;
        background-color: var(--primary-color);
    }
    #nav-centered {
        height: var(--nav-bar-height);
        background-color: var(--secondary-color);
        width: 100vw;
    }
    .nav-bar {
        display: flex;
        width: fit-content;
        height: inherit;
        margin: auto;
        justify-content: center;
        align-items: center;
    }
    .nav-bar a {
        color: white;
        text-decoration: none;
        display: flex;
        height: inherit;
        padding: 0vw 5vw;
        cursor: pointer;
        align-items: center;
        justify-content: center;
    }
    .separator {
        border-right: 1px solid black;
    }
    .selected {
        background-color: var(--selection-color);
        border-radius: 15px;
    }
</style>

<script>
    import { navigating } from '$app/stores';
    import { invoke } from '@tauri-apps/api/tauri';
    import { faFilter, faMagnifyingGlass, faEllipsisVertical } from '@fortawesome/free-solid-svg-icons'
    import { onMount } from "svelte";
    import Fa from 'svelte-fa'
    import store from "$lib/store.js";

    onMount(async () => {
        let library = await invoke('get_lib');
        store.update(json => {
            json.library = library;
            return json;
        });
    });

    // Redirect Manager
    $: if($navigating) {
        document.body.scrollTop;
        page_check()
    };
    let selected_valid_links = ["/library", "/updates", "/browse", "/more"];
    function page_check() {
        if (selected_valid_links.includes($navigating.to.url.pathname)) {
            // removes selected first from all
            selected_valid_links.forEach(link => {
                document.getElementById(link).classList.remove("selected");
            });
            document.getElementById($navigating.to.url.pathname).classList.add("selected");
        }


        if ($navigating.to.url.pathname.includes("reader")) {
            document.getElementById("snackbar").style.display = "none";
            document.getElementById("body").style.height = "100vh";
            document.getElementById("nav-centered").style.display = "none";
        } else {
            document.getElementById("snackbar").style.display = "block";
            document.getElementById("body").style.height = null;
            document.getElementById("nav-centered").style.display = "block";
        }
    }
    // PRIMARY APP COMMANDS & KEYS
    window.addEventListener('keydown', handleKeydown);
    function handleKeydown(event) {
        if (event.ctrlKey && event.key === 'a') {
        // Prevent the default browser action (select all)
        event.preventDefault();
        console.log("interesting")
        }
    }
</script>

<div id="snackbar">
    <button id="anime" class="snackbar-item">Manga</button>
    <button id="manga" class="snackbar-item">Anime</button>
    <button id="ln" class="snackbar-item">LN</button>
    <p class="snackbar-item" style="margin:0;padding:0;display:linline-flex;">Nothing here done</p>
    <div style="float: right; height: inherit">
        <!-- https://fontawesome.com/search -->
        <button id="ln" class="snackbar-item"><Fa icon={faMagnifyingGlass} /></button>
        <button id="ln" class="snackbar-item"><Fa icon={faFilter} /></button>
        <button id="ln" class="snackbar-item"><Fa icon={faEllipsisVertical} /></button>
    </div>
</div>

<div id="body">
    <slot></slot>
</div>

<div id="nav-centered">
    <nav class="nav-bar">
        <!-- todo: selected generated better -->
        <a id="/library" class="separator selected" href="/library">Library</a>
        <a id="/updates" class="separator" href="/updates">Updates</a>
        <a id="/browse" class="separator" href="/browse">Browse</a>
        <a id="/more" href="/more">More</a>
    </nav>
</div>


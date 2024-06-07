<script>
    import { navigating, page } from '$app/stores';
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { invoke } from '@tauri-apps/api/tauri';
    import { faFilter, faMagnifyingGlass, faEllipsisVertical, faArrowLeft, faHeart, faBook } from '@fortawesome/free-solid-svg-icons'
    import { faHeart as faOutlineHeart } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa'
    import store from "$lib/store.js";
    import { find_manga,in_lib,toggle_favorite } from "$lib/common.js";

    let library = [];
    onMount(async () => {
        library = await invoke('get_lib');
        let settings = await invoke('get_settings');
        store.update(json => {
            json.library = library;
            json.settings = settings;
            return json;
        });
    });

    let type = "manga";
    let nav = '';
    let selected_valid_links = ["/library", "/updates", "/browse", "/more"];
    $: path = $page.url.pathname;
    let scroll_memory = {};
    let in_manga = false;
    let in_browser = false;
    let manga_data = {
        favorited: false,
        data: {},
        back: "/"
    };

    // ----- REDIRECT MANAGER -----
    $: if($navigating) page_check();
    function page_check() {
        nav = $navigating.to.url.pathname;
        console.log(nav);
        let from = $navigating.from.url.pathname
        // Highlights nav bar items
        if (selected_valid_links.includes(nav)) {
            // removes selected first from all
            selected_valid_links.forEach(link => {
                document.getElementById(link).classList.remove("selected");
            });
            document.getElementById(nav).classList.add("selected");
        }

        // todo: Side scroll nav
        // if (from == "/browse") {
        //     if (document.getElementsByClassName("manga-section")) {
        //         //sets memory
        //         let arr = document.getElementsByClassName("manga-section");
        //         console.log(arr);
        //         // scroll_memory['side'] = arr.map(e => e.scrollLeft);
        //         //remembers
        //         if (!scroll_memory.hasOwnProperty('side')) {
        //             for (const e in document.getElementsByClassName("manga-section")) {
        //                 console.log(e.scrollLeft);
        //             }
        //         } else {
        //             console.log("huh")
        //         }
        //     }
        // } 
        // Is topmost for every nav, except for reader chapters
        if (!from.includes("reader")) {
            // sets memory
            scroll_memory[from] = document.getElementById('body').scrollTop;
            // remembers
            if (!scroll_memory.hasOwnProperty(nav)) {
                document.getElementById('body').scrollTop = 0;
            } else {
                // required for '/library', seemingly
                setTimeout(() => document.getElementById('body').scrollTop = scroll_memory[nav], 1);
            }
        }

        // --- MANGA ---
        // Changes the top nav for manga
        if (nav.includes("manga") && !nav.includes("reader")) {
            in_manga = true;
            switch (from) {
                case "/library":
                case "/updates":
                case "/browse":
                    manga_data.back = from;
            }
            manga_data.data = find_manga($navigating.to.params.manga);
            manga_data.favorited = in_lib(manga_data.data.id)
        } else {
            in_manga = false;
        }

        // --- READER ---
        // Removes snackbar and nav for reader
        if (nav.includes("reader")) {
            document.getElementById("snackbar").style.display = "none";
            document.getElementById("body").style.height = "100vh";
            document.getElementById("nav-centered").style.display = "none";
        } else {
            document.getElementById("body").style.height = null;
            document.getElementById("snackbar").style.display = "block";
            document.getElementById("nav-centered").style.display = "block";
        }
    }

    // ----- PRIMARY APP COMMANDS & KEYS ----- 
    window.addEventListener('keydown', handleKeydown);
    function handleKeydown(event) {
        if (event.ctrlKey && event.key === 'a') {
        // Prevent the default browser action (select all)
            event.preventDefault();
            console.log("interesting")
        }
    }

    // BACKEND CALLS
    async function toggle_fav() {
        manga_data.favorited = !manga_data.favorited;
        await toggle_favorite(manga_data.data);
    }
</script>

<div id="snackbar">
    <!-- left side -->
    {#if in_manga}
        <button class="snackbar-item" on:click={async () => goto(manga_data.back)}><Fa icon={faArrowLeft} /></button>
    {:else if nav.includes('add_sources')}
        <button class="snackbar-item" on:click={() => goto('/browse/')}><Fa icon={faArrowLeft} /></button>
    {:else}
        <button id="manga" class="snackbar-item {type=="manga"? 'selected':''}">Manga</button>
        <button id="anime" class="snackbar-item">Anime</button>
        <button id="ln" class="snackbar-item">LN</button>
        <p class="snackbar-item" style="margin:0;padding:0;display:linline-flex;">Nothing here done</p>
    {/if}
    
    <!-- right side -->
    <div class="snackbar-right">
        {#if in_manga}
            <button class="snackbar-item" on:click={async () => toggle_fav()}>
                {#if manga_data.favorited}
                <Fa icon={faHeart} />
                {:else}
                <Fa icon={faOutlineHeart} />
                {/if}
            </button>
            <button class="snackbar-item"><Fa icon={faFilter} /></button>
            <button class="snackbar-item"><Fa icon={faEllipsisVertical} /></button>
        {:else if nav.includes('browse')}
            <button class="snackbar-item" on:click={() => goto('/browse/add_sources')}><Fa icon={faBook} /></button>
        {:else}
            <!-- https://fontawesome.com/search -->
            <button class="snackbar-item"><Fa icon={faMagnifyingGlass} /></button>
            <button class="snackbar-item"><Fa icon={faFilter} /></button>
            <button class="snackbar-item"><Fa icon={faEllipsisVertical} /></button>
        {/if}
    </div>
</div>

<div id="body">
    <slot></slot>
</div>

<div id="nav-centered">
    <nav class="nav-bar">
        <a id="/library" class="{path=='/library'? 'selected' : ''}" href="/library">Library</a>
        <a id="/updates" class="{path=='/updates'? 'selected' : ''}" href="/updates">Updates</a>
        <a id="/browse" class="{path.includes('/browse')? 'selected' : ''}" href="/browse">Browse</a>
        <a id="/more" class="{path=='/more'? 'selected' : ''}" href="/more">More</a>
    </nav>
</div>

<style>
    :root {
        --snackbar-height: 25px;
        --nav-bar-height: 50px;
        --primary-color: #1a1a1a;
        --secondary-color: #330000;
        --selection-color: maroon;
        --secondary-color-transparent: rgba(51, 0, 0, 0.5);
    }
    #snackbar {
        height: var(--snackbar-height);
        background-color: var(--secondary-color);
    }
    .snackbar-item {
        display: inline-flex;
        align-items: center;
        background-color: transparent;
        /* background-color: green; */
        border: 0;
        color: white;
        font-size: medium;
        padding: 0 10px;
        height: inherit;
    }
    button.snackbar-item:hover {
        background-color: var(--selection-color);
    }
    .snackbar-right {
        float: right; 
        height: inherit
    }
    #body {
        height: calc(100vh - var(--snackbar-height) - var(--nav-bar-height));
        color: white;
        overflow: scroll;
        background-color: var(--primary-color);
    }
    /* Likely lags on firefox */
    #body::-webkit-scrollbar {
        display: none; 
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
        padding: 0vw 3vw;
        cursor: pointer;
        align-items: center;
        justify-content: center;
        border-radius: 15px;
        margin: 0 5px;
    }
    .nav-bar a:hover {
        background-color: var(--selection-color);
    }
    .selected {
        background-color: var(--selection-color);
    }
</style>
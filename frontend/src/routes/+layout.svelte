<script>
    import { navigating, page } from '$app/stores';
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import store from "$lib/store.js";
    // import { 
    //     find_manga,
    //     in_lib as in_manga_lib, 
    //     toggle_favorite as toggle_manga_favorite, 
    //     update_lib as manga_update
    // } from "$lib/manga_common.js";
    // import { 
    //     find_ln, 
    //     in_lib as in_ln_lib, 
    //     toggle_favorite as toggle_ln_favorite 
    // } from "$lib/ln_common.js";
    import logo from "$lib/logo.png"
    import Snackbar from "./snackbar.svelte";

    let manga_library = [];
    let ln_library = [];
    let type = "manga";
    let loading = true;
    onMount(async () => {
        // GET LIB
        manga_library = await invoke('get_manga_lib');
        ln_library = await invoke('get_ln_lib');
        let settings = await invoke('get_settings');
        store.update(json => {
            json.manga_library = manga_library;
            json.ln_library = ln_library;
            json.settings = settings;
            json.media_screen = type;
            return json;
        });
        // GET AND SET SETTINGS, if available
        if (settings.hasOwnProperty("app_colors")) {
            if (settings['app_colors'].hasOwnProperty("primary")) {
                document.documentElement.style.setProperty('--primary-color', `#${settings['app_colors'].primary}`); 
            }
            if (settings['app_colors'].hasOwnProperty("secondary")) {
                document.documentElement.style.setProperty('--secondary-color', `#${settings['app_colors'].secondary}`); 
            }
            if (settings['app_colors'].hasOwnProperty("selection")) {
                document.documentElement.style.setProperty('--selection-color', `#${settings['app_colors'].selection}`); 
            }
            if (settings['app_colors'].hasOwnProperty("text")) {
                document.documentElement.style.setProperty('--text-color', `#${settings['app_colors'].text}`); 
            }
        }
        if (settings.hasOwnProperty("library_widths")) {
            if (settings["library_widths"].hasOwnProperty("manga")) {
                document.documentElement.style.setProperty('--lib-manga-width', `${settings['library_widths'].manga}px`); 
            }
            if (settings["library_widths"].hasOwnProperty("ln")) {
                document.documentElement.style.setProperty('--lib-ln-width', `${settings['library_widths'].ln}px`); 
            }
            // if (settings["library_widths"].hasOwnProperty("anime")) {
                
            // }
        }
        loading = false;
    });

    let nav = '';
    let from = '/';
    let param_id = '';
    let selected_valid_links = ["library", "updates", "browse", "more"];
    $: path = $page.url.pathname;
    let scroll_memory = {};

    // ----- REDIRECT MANAGER -----
    $: if($navigating) page_check();
    function page_check() {
        nav = $navigating.to.url.pathname;
        if (nav != $navigating.from.url.pathname) {
            from = $navigating.from.url.pathname
        }
        console.log(nav);

        // default is on, other navs can change this
        snack_nav_on();

        // Hides nav bar when not selected
        if (!selected_valid_links.some(link => nav.includes(link))) {
            only_snackbar_on();
        }
        
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

        // Gets the param id for snackbar
        if (nav.includes("manga/") && !nav.includes("reader")) {
            param_id = $navigating.to.params.manga;
        }
        if (nav.includes("ln/") && !nav.includes("reader")) {
            param_id = $navigating.to.params.ln;
        }
        
        // --- READER ---
        // Removes snackbar and nav for reader
        if (nav.includes("reader")) {
            snack_nav_off();
        }
    }

    // ----- PRIMARY APP COMMANDS & KEYS ----- 
    function handleKeydown(event) {
        if (event.ctrlKey && event.key === 'm') {
            event.preventDefault();
            change_media("manga");
            goto('/manga_library');
        }
        if (event.ctrlKey && event.key === 'l') {
            event.preventDefault();
            change_media("ln");
            goto('/ln_library');
        }
    }

    // BODY, NAV, and SNACKBAR SIZES
    function snack_nav_off() {
        document.getElementById("snackbar").style.display = "none";
        document.getElementById("body").style.height = "100vh";
        document.getElementById("nav-centered").style.display = "none";
    }
    function snack_nav_on() {
        document.documentElement.style.setProperty('--nav-bar-height', '50px');
        document.getElementById("body").style.height = null;
        document.getElementById("snackbar").style.display = "block";
        document.getElementById("nav-centered").style.display = "block";
    }
    function only_snackbar_on() {
        document.documentElement.style.setProperty('--nav-bar-height', '0px');
        document.getElementById("nav-centered").style.display = "none";
        document.getElementById("body").style.height = "calc(100vh - var(--snackbar-height));";
        document.getElementById("snackbar").style.display = "block";
    }

    // Keeps the bottom nav at the bottom of the screen (keyboards move it)
	$: outerHeight = 0
    $: if (outerHeight) {
        console.log(outerHeight);
        document.documentElement.style.setProperty('--body-height', `calc(${outerHeight}px - var(--snackbar-height) - var(--nav-bar-height))`);
    }
</script>
<svelte:window bind:outerHeight on:keydown={handleKeydown} />

<div id="splashscreen" style="display: {loading? 'flex' : 'none'}">
    <img src={logo} alt="logo">
</div>

<div id="snackbar">
    <Snackbar {nav} {from} {param_id} />
</div>

<div id="body">
    <slot></slot>
</div>

<div id="nav-centered">
    <nav class="nav-bar">
        <!-- TODO: merge with more when possible (for History, stats?, settings) -->
        <a id="/library" class="{path.includes('library')? 'selected' : ''}" 
            href="/{type}_library">
            Library
        </a>
        <a id="/updates" class="{path=='/updates'? 'selected' : ''}" href="/updates">Updates</a>
        <a id="/browse" class="{path.includes('/browse')? 'selected' : ''}" href="/browse">Browse</a>
        <a id="/more" class="{path=='/more'? 'selected' : ''}" href="/more">More</a>
    </nav>
</div>

<style>
    #splashscreen {
        position: fixed;
        width: 100vw;
        height: 100vh;
        background-color: #2B76AF;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    #splashscreen img {
        /* margin: auto; */
        height: 20vh;
        width: auto;
    }
    :root {
        --snackbar-height: 35px;
        --nav-bar-height: 50px;
        --body-height: calc(100vh - var(--snackbar-height) - var(--nav-bar-height));

        --primary-color: #1a1a1a;
        --secondary-color: #330000;
        --selection-color: #800000;
        --text-color: white;

        --lib-manga-width: 100px;
        --lib-manga-height: calc(var(--lib-manga-width) *1.5);

        --lib-ln-width: 100px;
        --lib-ln-height: calc(var(--lib-ln-width) *1.5);
    }
    #snackbar {
        height: var(--snackbar-height);
        background-color: var(--secondary-color);
    }
    #body {
        height: var(--body-height);
        color: var(--text-color);
        overflow: scroll;
        background-color: var(--primary-color);
        scrollbar-width: none;
        -ms-overflow-style: none;
    }
    /* Likely lags on firefox */
    /* #body::-webkit-scrollbar {
        display: none; 
    } */
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
        color: var(--text-color);
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
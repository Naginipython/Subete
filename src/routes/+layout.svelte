<script>
    import { navigating, page } from '$app/stores';
    import Snackbar from './snackbar.svelte';
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { setup } from "$lib/common.js";
    import appHist from "$lib/history.js";
    import store from "$lib/store.js";
    import logo from "$lib/logo.png"

    // Gets data from backend, sets settings
    let media_screen = "manga";
    let loading = true;
    onMount(async () => {
        // GET LIB
        await setup(media_screen);
        goto('/library');
        loading = false;
    });
    store.subscribe(json => {
        media_screen = json.media_screen;
    });

    let nav = '';
    let selected_valid_links = ["/library", "/updates", "/browse", "/more"];
    $: path = $page.url.pathname;
    let scroll_memory = {};
    let from = "/";

    // ----- REDIRECT MANAGER -----
    $: if($navigating) page_check();
    function page_check() {
        nav = $navigating.to.url.pathname;
        appHist.pushState(nav);
        appHist.print();
        if (nav != $navigating.from.url.pathname) {
            from = $navigating.from.url.pathname
        }
        console.log(nav);

        // default is on, other navs can change this
        snack_nav_on();

        // Hides nav bar when not selected
        if (!selected_valid_links.some(link => nav == link)) {
            only_snackbar_on();
        } else {
            // Prevents history from getting too big
            // appHist.reset();
            // appHist.pushState(nav);
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
        // Is topmost for every nav, except for items
        if (!from.includes("/item/")) {
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

        // --- VIEWER ---
        // Removes snackbar and nav for viewer
        if (nav.includes("viewer")) {
            snack_nav_off();
        }
    }
    function change_media(media) {
        media_screen = media;
        store.update(json => {
            json.media_screen = media_screen;
            return json;
        });
    }

    // ----- PRIMARY APP COMMANDS & KEYS ----- 
    function handleKeydown(event) {
        if (event.ctrlKey && event.key === 'm') {
            event.preventDefault();
            change_media("manga");
            goto('/library');
        }
        if (event.ctrlKey && event.key === 'l') {
            event.preventDefault();
            change_media("ln");
            goto('/library');
        }
        if (event.ctrlKey && event.key === 'a') {
            event.preventDefault();
            change_media("anime");
            goto('/library');
        }
    }

    // BODY, NAV, and SNACKBAR SIZES
    let is_nav_off = false;
    function snack_nav_off() {
        is_nav_off = true;
        document.getElementById("snackbar").style.display = "none";
        document.getElementById("body").style.height = "100vh";
        document.getElementById("nav-centered").style.display = "none";
    }
    function snack_nav_on() {
        is_nav_off = false;
        document.documentElement.style.setProperty('--nav-bar-height', '55px');
        document.getElementById("body").style.height = null;
        document.getElementById("snackbar").style.display = "block";
        document.getElementById("nav-centered").style.display = "block";
    }
    function only_snackbar_on() {
        is_nav_off = true;
        document.documentElement.style.setProperty('--nav-bar-height', '0px');
        document.getElementById("nav-centered").style.display = "none";
        document.getElementById("body").style.height = "calc(100vh - var(--snackbar-height));";
        document.getElementById("snackbar").style.display = "block";
    }

    // Keeps the bottom nav at the bottom of the screen (keyboards move it)
	$: outerHeight = 0
    $: if (outerHeight) {
        document.documentElement.style.setProperty('--body-height', `calc(${outerHeight}px - var(--snackbar-height) - var(--nav-bar-height))`);
    }
</script>
<svelte:window bind:outerHeight on:keydown={handleKeydown} />

<div id="splashscreen" style="display: {loading? 'flex' : 'none'}">
    <img src={logo} alt="logo">
</div>

<Snackbar {is_nav_off} {nav} {from} />

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
        --snackbar-height: 45px;
        --nav-bar-height: 55px;
        --body-height: calc(100vh - var(--snackbar-height) - var(--nav-bar-height));

        --primary-color: #1a1a1a;
        --secondary-color: #330000;
        --selection-color: #800000;
        --text-color: white;

        --lib-manga-width: 100px;
        --lib-manga-height: calc(var(--lib-manga-width) *1.5);

        --lib-ln-width: 100px;
        --lib-ln-height: calc(var(--lib-ln-width) *1.5);

        --lib-anime-width: 100px;
        --lib-anime-height: calc(var(--lib-anime-width) *1.5);
    }
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
<!-- <script>
    import { invoke } from "@tauri-apps/api/core";

    // let url = 'https://server.elscione.com/Officially%20Translated%20Light%20Novels/Classroom%20of%20the%20Elite%20-%20Year%202/Classroom%20of%20the%20Elite%20-%20Year%202%20-%20Volume%2009%20%5BSeven%20Seas%5D%5BKobo_LNWNCentral%5D.pdf';
    let url="https://raw.githubusercontent.com/vinodnimbalkar/svelte-pdf/369db2f9edbf5ab8c87184193e1404340729bb3a/public/sample.pdf";
    async function test() {
        await invoke("download_ln", {url});
    }
</script>
<button on:click={async () => test()}>download me</button> -->
<script>
    import { navigating, page } from '$app/stores';
    import { goto } from "$app/navigation";
    import store from "$lib/store.js";
    import Fa from 'svelte-fa';
    import { 
        faFilter, faMagnifyingGlass, 
        faEllipsisVertical, faArrowLeft, 
        faHeart, faBook, faRotateRight 
    } from '@fortawesome/free-solid-svg-icons'
    import { faHeart as faOutlineHeart } from '@fortawesome/free-regular-svg-icons';
    import appHist from "$lib/history.js";
    import {
        find_item,
        in_lib,
        toggle_favorite,
        update
    } from "$lib/common.js";

    // Vars
    export let nav = '';
    let media_screen = "manga";
    export let is_nav_off;
    let in_manga = false;
    let manga_data = {
        favorited: false,
        data: {}
    };
    let in_anime = false;
    let anime_data = {
        favorited: false,
        data: {}
    };
    let in_ln= false;
    let ln_data = {
        favorited: false,
        data: {}
    };

    // Snackbar navigation manager
    $: if($navigating) page_check();
    async function page_check() {
        // --- MANGA ---
        // Changes the top nav for manga
        if (nav.includes("/manga/")) {
            in_manga = true;
            manga_data.data = await find_item("manga", $navigating.to.params.plugin, $navigating.to.params.id);
            manga_data.favorited = in_lib("manga", manga_data.data.id)
        } else {
            in_manga = false;
        }

        // --- ANIME ---
        // Changes the top nav for anime
        if (nav.includes("anime/")) {
            in_anime = true;
            anime_data.data = await find_item("anime", $navigating.to.params.plugin, $navigating.to.params.id);
            anime_data.favorited = in_lib("anime", anime_data.data.id)
        } else {
            in_anime = false;
        }

        // --- LN ---
        // Changes the top nav for ln
        if (nav.includes("ln/")) {
            in_ln = true;
            ln_data.data = await find_item("ln", $navigating.to.params.plugin, $navigating.to.params.id);
            ln_data.favorited = in_lib("ln", ln_data.data.id)
        } else {
            in_ln = false;
        }
    }

    // BACKEND CALLS
    async function toggle_manga_fav() {
        manga_data.favorited = !manga_data.favorited;
        await toggle_favorite("manga", manga_data.data);
    }
    async function toggle_ln_fav() {
        ln_data.favorited = !ln_data.favorited;
        await toggle_favorite("ln", ln_data.data);
    }
    async function toggle_anime_fav() {
        anime_data.favorited = !anime_data.favorited;
        await toggle_favorite("anime", anime_data.data);
    }
    async function update_lib() {
        update(media_screen);
    }

    // MEDIA TYPE CHANGE
    function change_media(media) {
        media_screen = media;
        store.update(json => {
            json.media_screen = media_screen;
            return json;
        });
    }
    store.subscribe(json => {
        media_screen = json.media_screen;
    });
</script>

<div id="snackbar">
    <!-- left side -->
    {#if in_manga || in_ln || in_anime}
        <button class="snackbar-item" on:click={async () => goto(appHist.back())}><Fa icon={faArrowLeft} /></button>
    {:else if is_nav_off}
        <button class="snackbar-item" on:click={() => goto(appHist.back())}><Fa icon={faArrowLeft} /></button>
    {:else}
        <!-- TODO: make work -->
        <button id="manga" class="media {media_screen=="manga"? 'selected':''}" on:click={() => change_media("manga")}>Manga</button>
        <button id="anime" class="media {media_screen=="anime"? 'selected':''}" on:click={() => change_media("anime")}>Anime</button>
        <button id="ln" class="media {media_screen=="ln"? 'selected':''}" on:click={() => change_media("ln")}>LN</button>
    {/if}
    
    <!-- right side -->
    <div class="snackbar-right">
        {#if in_manga}
            <button class="snackbar-item" on:click={async () => toggle_manga_fav()}>
                {#if manga_data.favorited}
                <Fa icon={faHeart} />
                {:else}
                <Fa icon={faOutlineHeart} />
                {/if}
            </button>
            <button class="snackbar-item"><Fa icon={faFilter} /></button>
            <button class="snackbar-item"><Fa icon={faEllipsisVertical} /></button>
        {:else if in_ln}
            <button class="snackbar-item" on:click={async () => toggle_ln_fav()}>
                {#if ln_data.favorited}
                <Fa icon={faHeart} />
                {:else}
                <Fa icon={faOutlineHeart} />
                {/if}
            </button>
            <button class="snackbar-item"><Fa icon={faFilter} /></button>
            <button class="snackbar-item"><Fa icon={faEllipsisVertical} /></button>
        {:else if in_anime}
            <button class="snackbar-item" on:click={async () => toggle_anime_fav()}>
                {#if anime_data.favorited}
                <Fa icon={faHeart} />
                {:else}
                <Fa icon={faOutlineHeart} />
                {/if}
            </button>
            <button class="snackbar-item"><Fa icon={faFilter} /></button>
            <button class="snackbar-item"><Fa icon={faEllipsisVertical} /></button>
        {:else if nav.includes('browse')}
            <button class="snackbar-item" on:click={() => goto('/browse/add_sources')}><Fa icon={faBook} /></button>
        {:else if nav.includes('update')}
            <button class="snackbar-item" on:click={async () => update_lib()}><Fa icon={faRotateRight} /></button>
        {:else}
            <!-- https://fontawesome.com/search -->
            <!-- TODO: make work -->
            <button class="snackbar-item"><Fa icon={faMagnifyingGlass} /></button>
            <button class="snackbar-item"><Fa icon={faFilter} /></button>
            <button class="snackbar-item"><Fa icon={faEllipsisVertical} /></button>
        {/if}
    </div>
</div>

<style>
    #snackbar {
        height: var(--snackbar-height);
        background-color: var(--secondary-color);
    }
    .media {
        display: inline-flex;
        align-items: center;
        background-color: transparent;
        border: 0;
        color: var(--text-color);
        font-size: medium;
        padding: 0 8px;
        height: inherit;
        justify-content: center;
        width: 60px;
    }
    .media:hover {
        background-color: var(--selection-color);
    }
    .snackbar-item {
        display: inline-flex;
        align-items: center;
        background-color: transparent;
        border: 0;
        color: var(--text-color);
        font-size: medium;
        padding: 0 8px;
        height: inherit;
        justify-content: center;
        width: var(--snackbar-height);
    }
    button.snackbar-item:hover {
        background-color: var(--selection-color);
    }
    .snackbar-right {
        float: right; 
        height: inherit
    }
    .selected {
        background-color: var(--selection-color);
    }
</style>
<script>
    import { navigating } from '$app/stores';
    import { goto } from "$app/navigation";
    import store from "$lib/store.js";
    import { 
        find_manga,
        in_lib as in_manga_lib, 
        toggle_favorite as toggle_manga_favorite, 
        update_lib as manga_update
    } from "$lib/manga_common.js";
    import { 
        find_ln, 
        in_lib as in_ln_lib, 
        toggle_favorite as toggle_ln_favorite 
    } from "$lib/ln_common.js";
    import { 
        faFilter, faMagnifyingGlass, 
        faEllipsisVertical, faArrowLeft, 
        faHeart, faBook, faRotateRight 
    } from '@fortawesome/free-solid-svg-icons'
    import { faHeart as faOutlineHeart } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa';

    export let nav = '';
    export let from = "/";
    export let param_id = "";
    let back = "/";
    let in_manga = false;
    let manga_data = {
        favorited: false,
        data: {}
    };
    let in_ln= false;
    let ln_data = {
        favorited: false,
        data: {}
    };
    let type = "manga";
    $: if(nav) {
        // --- MANGA ---
        // Changes the top nav for manga
        if (nav.includes("manga/") && !nav.includes("reader")) {
            in_manga = true;
            if (from.includes('library') || from=='/updates' || from=='/browse') {
                back = from;
            }
            manga_data.data = find_manga(param_id);
            manga_data.favorited = in_manga_lib(manga_data.data.id)
        } else {
            in_manga = false;
        }
        // --- LN ---
        // Changes the top nav for ln
        if (nav.includes("ln/") && !nav.includes("reader")) {
            in_ln = true;
            if (from.includes('library') || from=='/updates' || from=='/browse') {
                back = from;
            }
            ln_data.data = find_ln(param_id);
            ln_data.favorited = in_ln_lib(ln_data.data.id)
        } else {
            in_ln = false;
        }
    }

    // BACKEND CALLS
    async function toggle_manga_fav() {
        console.log(manga_data);
        manga_data.favorited = !manga_data.favorited;
        await toggle_manga_favorite(manga_data.data);
        console.log(manga_data);
    }
    async function toggle_ln_fav() {
        ln_data.favorited = !ln_data.favorited;
        await toggle_ln_favorite(ln_data.data);
    }
    async function update_lib() {
        switch (media_screen) {
            case "manga": manga_update(); break;
            case "anime": /*anime_update();*/ break;
            case "ln": /*ln_update();*/ break;
        }
    }

    // MEDIA TYPE CHANGE
    function change_media(media) {
        type = media;
        store.update(json => {
            json.media_screen = type;
            return json;
        });
        if (nav.includes('library') || nav == '') {
            goto(`/${type}_library`);
        }
    }
</script>

<!-- left side -->
{#if in_manga || in_ln}
    <button class="snackbar-item" on:click={async () => goto(back)}><Fa icon={faArrowLeft} /></button>
{:else if nav.includes('add_sources') || nav.includes('settings')}
    <button class="snackbar-item" on:click={() => goto(from)}><Fa icon={faArrowLeft} /></button>
{:else}
    <!-- TODO: make work -->
    <button id="manga" class="snackbar-item {type=="manga"? 'selected':''}" on:click={() => change_media("manga")}>Manga</button>
    <button id="anime" class="snackbar-item {type=="anime"? 'selected':''}">Anime</button>
    <button id="ln" class="snackbar-item {type=="ln"? 'selected':''}" on:click={() => change_media("ln")}>LN</button>
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
{:else if nav.includes('browse')}
    <button class="snackbar-item" on:click={() => goto('/browse/add_sources')}><Fa icon={faBook} /></button>
{:else if nav.includes('update')}
    <!-- <button class="snackbar-item" on:click={async () => update_lib()}><Fa icon={faRotateRight} /></button> -->
{:else}
    <!-- https://fontawesome.com/search -->
    <!-- TODO: make work -->
    <button class="snackbar-item"><Fa icon={faMagnifyingGlass} /></button>
    <button class="snackbar-item"><Fa icon={faFilter} /></button>
    <button class="snackbar-item"><Fa icon={faEllipsisVertical} /></button>
{/if}
</div>

<style>
    .snackbar-item {
        display: inline-flex;
        align-items: center;
        background-color: transparent;
        /* background-color: green; */
        border: 0;
        color: var(--text-color);
        font-size: medium;
        padding: 0 8px;
        height: inherit;
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
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
    import { 
        find_manga,
        in_lib as in_manga_lib, 
        toggle_favorite as toggle_manga_favorite, 
        update_lib as manga_update
    } from "$lib/manga_common.js";
    import { 
        find_anime,
        in_lib as in_anime_lib, 
        toggle_favorite as toggle_anime_favorite, 
        update_lib as anime_update
    } from "$lib/anime_common.js";
    import { 
        find_ln, 
        in_lib as in_ln_lib, 
        toggle_favorite as toggle_ln_favorite 
    } from "$lib/ln_common.js";

    // Vars
    export let nav = '';
    export let from = "/";
    let media_screen = "manga";
    export let is_nav_off;
    let back = "/";
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
    function page_check() {
        console.log(nav);

        // --- MANGA ---
        // Changes the top nav for manga
        if (nav.includes("manga/") && !nav.includes("reader")) {
            in_manga = true;
            if (from.includes('library') || from=='/updates' || from=='/browse') {
                back = from;
            }
            manga_data.data = find_manga($navigating.to.params.plugin, $navigating.to.params.manga);
            manga_data.favorited = in_manga_lib(manga_data.data.id)
        } else {
            in_manga = false;
        }

        // --- ANIME ---
        // Changes the top nav for anime
        if (nav.includes("anime/") && !nav.includes("reader")) {
            in_anime = true;
            if (from.includes('library') || from=='/updates' || from=='/browse') {
                back = from;
            }
            anime_data.data = find_anime($navigating.to.params.plugin, $navigating.to.params.anime);
            anime_data.favorited = in_anime_lib(anime_data.data.id)
        } else {
            in_anime = false;
        }

        // --- LN ---
        // Changes the top nav for ln
        if (nav.includes("ln/") && !nav.includes("reader")) {
            in_ln = true;
            if (from.includes('library') || from=='/updates' || from=='/browse') {
                back = from;
            }
            ln_data.data = find_ln($navigating.to.params.ln);
            ln_data.favorited = in_ln_lib(ln_data.data.id)
        } else {
            in_ln = false;
        }
    }

    // BACKEND CALLS
    async function toggle_manga_fav() {
        manga_data.favorited = !manga_data.favorited;
        await toggle_manga_favorite(manga_data.data);
    }
    async function toggle_ln_fav() {
        ln_data.favorited = !ln_data.favorited;
        await toggle_ln_favorite(ln_data.data);
    }
    async function toggle_anime_fav() {
        anime_data.favorited = !anime_data.favorited;
        await toggle_anime_favorite(anime_data.data);
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
        media_screen = media;
        store.update(json => {
            json.media_screen = media_screen;
            return json;
        });
        if (nav.includes('library') || nav == '') {
            goto(`/${media_screen}_library`);
        }
    }
    store.subscribe(json => {
        media_screen = json.media_screen;
    });
</script>

<div id="snackbar">
    <!-- left side -->
    {#if in_manga || in_ln || in_anime}
        <button class="snackbar-item" on:click={async () => goto(back)}><Fa icon={faArrowLeft} /></button>
    {:else if is_nav_off}
        <button class="snackbar-item" on:click={() => goto(from)}><Fa icon={faArrowLeft} /></button>
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
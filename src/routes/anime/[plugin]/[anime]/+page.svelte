<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { navigating } from '$app/stores';
    import { faEllipsisVertical, faBookmark, faAnglesDown, faCircleCheck, faSquareCheck, faCheck, faArrowTurnDown, faXmark } from '@fortawesome/free-solid-svg-icons'
    import { faCircleDown, faBookmark as faOutlineBookmark, faSquare } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa'
    import store from "$lib/store.js"
    import { find_anime } from "$lib/anime_common.js";
    import { Moon } from 'svelte-loading-spinners';
    import "$lib/css/listItems.css";

    export let data;

    let anime = {};
    let loading = false;
    let error = "";

    onMount(async () => {
        loading = true;
        if (anime['episodes'].length == 0) {
            let updated_anime = await invoke('get_anime_episodes', { anime });
            if (!updated_anime.hasOwnProperty("error")) {
                updated_anime['episodes'].sort((a,b) => b.number-a.number);
                anime.studio = updated_anime.studio;
                anime.status = updated_anime.artist;
                anime.description = updated_anime.description;
                anime.episodes = updated_anime.episodes;
            } else {
                error = updated_anime.error;
            }
        } else {
            anime['episodes'] = anime['episodes'].sort((a,b) => b.number-a.number);
        }
        loading = false;
    });
    
    store.subscribe(async (json) => {
        // gets anime search details
        anime = find_anime(data.plugin, data.id);
    });

    // CHAPTER OPTION BUTTONS
    function toggle_complete(index) {
        if (anime['episodes'][index].completed) {
            anime['episodes'][index].completed = false;
            anime['episodes'][index].page = 1;
        } else {
            anime['episodes'][index].completed = true;
        }
        invoke('update_anime_lib', { item: anime });
    }
    function check_below(index) {
        for (let i = index+1; i < anime['episodes'].length; i++) {
            anime['episodes'][i].completed = true;
        }
        invoke('update_anime_lib', { item: anime });
    }
    function remove_page(index) {
        anime['episodes'][index].page = 1;
        invoke('update_anime_lib', { item: anime });
    }
    let opened = [];
    function show_options(index) {
        if (opened.includes(index)) {
            let i = opened.indexOf(index);
            if (i != -1) opened.splice(i, 1);
            document.getElementById(`i-${index}`).style.display = "none";
            document.getElementById(`o-${index}`).style.display = "flex";
        } else {
            opened.push(index);
            document.getElementById(`i-${index}`).style.display = "flex";
            document.getElementById(`o-${index}`).style.display = "none";
        }
    }
</script>

<div id="header">
    <div id="img-wrapper">
        <img id="img" src="{anime.img}" alt="{anime.title}"/>
    </div>
    <div id="header-content">
        <div id="text">
            <h3>{anime.title}</h3>
            <p>Status: {anime.status}</p>
            <p>Plugin: {anime.plugin}</p>
            <!-- TODO: fix desc scrolling -->
            <div id="desc"><p>{anime.description}</p></div>
        </div>
    </div>
</div>

<h3 class="ep-num">{anime['episodes'].length} episodes</h3>

<!-- Loading icon -->
<div style="margin: auto; width: fit-content; display: {loading? 'block' : 'none'}">
    <Moon color="var(--selection-color)" size="30" />
</div>

{#if error != ""}
    <p style="color: red; width: inherit; text-align: center; padding: 0; margin: 0">{error}</p>
{/if}

{#each anime['episodes'] as c, i}
<div class="episode-item" style="{anime['episodes'][i].completed? 'color: grey' : ''}">
    <!-- Main Chapter button -->
    <button class="episode-link" on:click={() => goto(`/anime/${data.plugin}/${data.id}/reader/${i}`)}>
        <p>
            {#if c.title == "" || c.title.toLowerCase() == "episode "+c.number} Episode {c.number}
            {:else} Episode {c.number} - {c.title}
            {/if}
        </p>
        <div class="chapter-lower">
            <p>date - group</p>
            <!-- {#if anime['episodes'][i].page-1 != 0 && !anime['chapters'][i].completed}
                <p class="progress">&emsp;(page: {manga['chapters'][i].page})</p>
            {/if} -->
        </div>
    </button>
    <!-- options menu -->
    <div class="ep-btns">
        <!-- simple check icon -->
        <button id="check" class="episode-btn" on:click={() => toggle_complete(i)}>
            {#if anime['episodes'][i].completed} <Fa icon={faXmark} />
            {:else} <Fa icon={faCheck} />
            {/if}
        </button>
        <!-- expanded options menu -->
        <div id="i-{i}" style="display:none">
            <!-- TODO: make work -->
            <button id="bookmark" class="episode-btn"><Fa icon={faOutlineBookmark} /></button>
            <button id="check-below" class="episode-btn" on:click={() => check_below(i)}><Fa icon={faAnglesDown} /></button>
            <button id="download" class="episode-btn"><Fa icon={faCircleDown} /><!--<Fa icon={faCircleCheck} />--></button>
            <button id="select" class="episode-btn"><Fa icon={faSquare} /><!--<Fa icon={faSquareCheck} />--></button>
            {#if anime['episodes'][i].page-1 != 0}
            <button id="uncheck" class="episode-btn" on:click={() => remove_page(i)}>
                <Fa icon={faXmark} />
            </button>
            {/if}
            <button id="options-return" class="episode-btn" on:click={() => show_options(i)}>
                <Fa icon={faArrowTurnDown} />
            </button>
        </div>
        <!-- options expantion button -->
        <div id="o-{i}" style="display:flex">
            <button id="options" class="episode-btn" on:click={() => show_options(i)}>
                <Fa icon={faEllipsisVertical} />
            </button>
        </div>
    </div>
</div>
{/each}

<style>
    :root {
        --anime-img-width: 25vw;
    }
    #header {
        --anime-img-width: 25vw;
        overflow: hidden;
        border-bottom: 1px solid black;
        display: inline-flex;
        height: calc(var(--anime-img-width) *1.5 + 20px);
        width: 100vw;
    }
    #header #img-wrapper {
        align-items: center;
    }
    #img-wrapper {
        width: var(--anime-img-width);
        height: calc(var(--anime-img-width) *1.5);
        border-radius: 10px;
        overflow: hidden;
        justify-content: center;
        align-items: center;
        margin: 10px;
        display: flex;
    }
    #img {
        height: 105%;
        width: auto;
        padding: 5px;
        justify-content: center;
    }
    #header-content {
        display: inline-flex;
        justify-content: left;
        overflow: hidden;
        width: calc(100% - var(--anime-img-width));
        padding-right: 10px;
        flex-direction: column;
    }
    #text {
        height: inherit;
        /* overflow: hidden; */
    }
    #text h3 {
        margin: 10px;
    }
    #text p {
        margin: 10px;
        font-size: smaller;
    }
    #desc {
        overflow: scroll;
        overflow-x: hidden;
    }
    .ep-num {
        padding: 10px;
        margin: 0;
    }
    .ep-btns {
        float: right;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        margin-right: 10px;
    }
    .episode-btn {
        display: inline-flex;
        align-items: center;
        background-color: transparent;
        border: 0;
        color: var(--text-color);
        font-size: large;
        height: inherit;
        margin: 0 2px;
        width: 25px;
    }
</style>
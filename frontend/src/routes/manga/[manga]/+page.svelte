<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { faEllipsisVertical, faBookmark, faAnglesDown, faCircleCheck, faSquareCheck, faCheck, faArrowTurnDown, faXmark } from '@fortawesome/free-solid-svg-icons'
    import { faCircleDown, faBookmark as faOutlineBookmark, faSquare } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa'
    import store from "$lib/store.js"
    import { find_manga } from "$lib/manga_common.js";
    import { Moon } from 'svelte-loading-spinners';

    export let data;

    let manga = {};
    let loading = false;

    // Adds to history when data is available
    $: if (Object.keys(manga).length != 0) {
        store.update(json => {
            json.manga_history.push(manga);
            return json;
        });
    }

    onMount(async () => {
        if (manga['chapters'].length == 0) {
            loading = true;
            let updated_manga = await invoke('get_manga_chapters', { manga });
            // let html = await invoke('fetch', {url: c.url});
            // manga['chapters'] = eval(c.getChapters + `getChapters(${manga}, ${JSON.stringify(html)})`);
            updated_manga['chapters'].sort((a,b) => b.number-a.number);
            manga.author = updated_manga.author;
            manga.artist = updated_manga.artist;
            manga.description = updated_manga.description;
            manga.chapters = updated_manga.chapters;
            loading = false;
        }
    });
    
    store.subscribe(async (json) => {
        // gets manga search details
        manga = find_manga(data.id);
    });

    // CHAPTER OPTION BUTTONS
    function toggle_complete(index) {
        if (manga['chapters'][index].completed) {
            manga['chapters'][index].completed = false;
            manga['chapters'][index].page = 1;
        } else {
            manga['chapters'][index].completed = true;
        }
        invoke('update_manga_lib', { item: manga });
    }
    function check_below(index) {
        for (let i = index+1; i < manga['chapters'].length; i++) {
            manga['chapters'][i].completed = true;
        }
        invoke('update_manga_lib', { item: manga });
    }
    function remove_page(index) {
        manga['chapters'][index].page = 1;
        invoke('update_manga_lib', { item: manga });
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
        <img id="img" src="{manga.img}" alt="{manga.title}"/>
    </div>
    <div id="header-content">
        <div id="text">
            <h3>{manga.title}</h3>
            <p>Author: {manga.authors}</p>
            <p>Plugin: {manga.plugin}</p>
            <!-- TODO: fix desc scrolling -->
            <div id="desc"><p>{manga.description}</p></div>
        </div>
    </div>
</div>

<!-- Loading icon -->
<div style="margin: auto; width: fit-content; display: {loading? 'block' : 'none'}">
    <Moon color="var(--selection-color)" size="30" />
</div>

<h3 class="chap-num">{manga['chapters'].length} chapters</h3>

{#each manga['chapters'] as c, i}
<div class="chapter" style="{manga['chapters'][i].completed? 'color: grey' : ''}">
    <!-- Main Chapter button -->
    <button class="chapter-link" on:click={() => goto(`/manga/${data.id}/reader/${i}`)}>
        <p>
            {#if c.title == "" || c.title.toLowerCase() == "chapter "+c.number} Chapter {c.number}
            {:else} Chapter {c.number} - {c.title}
            {/if}
        </p>
        <div class="chapter-lower">
            <p>date - group</p>
            {#if manga['chapters'][i].page-1 != 0 && !manga['chapters'][i].completed}
                <p class="progress">&emsp;(page: {manga['chapters'][i].page})</p>
            {/if}
        </div>
    </button>
    <!-- options menu -->
    <div class="chap-btns">
        <!-- simple check icon -->
        <button id="check" class="chapter-btn" on:click={() => toggle_complete(i)}>
            {#if manga['chapters'][i].completed} <Fa icon={faXmark} />
            {:else} <Fa icon={faCheck} />
            {/if}
        </button>
        <!-- expanded options menu -->
        <div id="i-{i}" style="display:none">
            <!-- TODO: make work -->
            <button id="bookmark" class="chapter-btn"><Fa icon={faOutlineBookmark} /></button>
            <button id="check-below" class="chapter-btn" on:click={() => check_below(i)}><Fa icon={faAnglesDown} /></button>
            <button id="download" class="chapter-btn"><Fa icon={faCircleDown} /><!--<Fa icon={faCircleCheck} />--></button>
            <button id="select" class="chapter-btn"><Fa icon={faSquare} /><!--<Fa icon={faSquareCheck} />--></button>
            {#if manga['chapters'][i].page-1 != 0}
            <button id="uncheck" class="chapter-btn" on:click={() => remove_page(i)}>
                <Fa icon={faXmark} />
            </button>
            {/if}
            <button id="options-return" class="chapter-btn" on:click={() => show_options(i)}>
                <Fa icon={faArrowTurnDown} />
            </button>
        </div>
        <!-- options expantion button -->
        <div id="o-{i}" style="display:flex">
            <button id="options" class="chapter-btn" on:click={() => show_options(i)}>
                <Fa icon={faEllipsisVertical} />
            </button>
        </div>
    </div>
</div>
{/each}

<style>
    :root {
        --manga-img-width: 25vw;
    }
    #header {
        overflow: hidden;
        border-bottom: 1px solid black;
        display: inline-flex;
        height: calc(var(--manga-img-width) *1.5 + 20px);
        width: 100vw;
    }
    #header #img-wrapper {
        align-items: center;
    }
    #img-wrapper {
        width: var(--manga-img-width);
        height: calc(var(--manga-img-width) *1.5);
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
        width: calc(100% - var(--manga-img-width));
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
    .chap-num {
        padding: 10px;
        margin: 0;
    }
    .chapter {
        width: 100vw;
        padding: 5px 0;
        overflow: auto;
        display: inline-flex;
        background-color: var(--primary-color);
        /* filter: brightness(0.95); */
    }
    .chapter:hover {
        background-color: var(--selection-color)
    }
    .chapter-link p {
        justify-content: left;
        width: inherit;
        padding: 0;
        margin: 0;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
    }
    @media only screen and (max-width: 550px) {
        .chapter-link p {
            font-size: medium;
        }
    }
    .chapter-link {
        text-align: left;
        cursor: pointer;
        display: inline-flex;
        justify-content: left;
        flex-direction: column;
        padding-left: 15px;
        border: 0;
        background-color: transparent;
        color: inherit;
        width: 100%;
        float: left;
        
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        /* width: idk; TODO */
    }
    .chapter-lower {
        padding-top: 5px;
        display: inline-flex;
    }
    .chapter-lower p {
        margin: 0; 
        padding: 0; 
        font-size: x-small;
    }
    .progress {
        color: grey;
    }
    .chap-btns {
        float: right;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        margin-right: 10px;
    }
    .chapter-btn {
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
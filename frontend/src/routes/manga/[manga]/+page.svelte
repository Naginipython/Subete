<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { goto } from "$app/navigation";
    import { faEllipsisVertical, faBookmark, faAnglesDown, faCircleCheck, faSquareCheck, faCheck, faArrowTurnDown, faXmark } from '@fortawesome/free-solid-svg-icons'
    import { faCircleDown, faBookmark as faOutlineBookmark, faSquare } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa'
    import store from "$lib/store.js"
    // import { getChapters } from "$lib/manga_sources/main.js";
    import { find_manga } from "$lib/common.js";

    export let data;

    let manga = {};

    // Adds to history when data is available
    $: if (Object.keys(manga).length != 0) {
        store.update(json => {
            json.history.push(manga);
            return json;
        });
    }
    
    store.subscribe(async (json) => {
        // gets manga search details
        manga = find_manga(data.id);

        // gets chapters, if needed
        if (manga['chapters'].length == 0) {
            // manga['chapters'] = await getChapters(manga.plugin, manga.id);
            // manga['chapters'].sort((a,b) => b.number-a.number);
            manga['chapters'] = await invoke('get_chapters', { source: manga.plugin, id: manga.id });
            manga['chapters'].sort((a,b) => b.number-a.number);
        }
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

<div id="header" >
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


<!-- TODO: loading icon -->
{#each manga['chapters'] as c, i}
<div class="chapter" style="{manga['chapters'][i].completed? 'color: grey' : ''}">
    <!-- Main Chapter button -->
    <button class="chapter-link" on:click={() => goto(`/manga/${data.id}/reader/${i}`)}>
        <p>
            {#if c.title == ""} Chapter {c.number}
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
    #header {
        padding: 10px;
        overflow: hidden;
        border: 1px solid black;
    }
    #img-wrapper {
        width: 200px;
        height: 300px;
        border-radius: 10px;
        overflow: hidden;
        justify-content: center;
        align-items: center;
        float: left;
    }
    #img {
        height: 105%;
        width: auto;
    }
    #header-content {
        display: flex;
        justify-content: left;
        float: right;
        overflow: hidden;
        width: calc(100% - 250px);
        padding-right: 10px;
        height: 300px;
    }
    #text {
        height: inherit;
        /* overflow: hidden; */
    }
    #text p {
        margin: 10px;
        font-size: smaller;
    }
    #desc {
        overflow: scroll;
        overflow-x: hidden;
    }
    .chapter {
        width: 100vw;
        padding: 5px 0;
        overflow: auto;
        display: inline-flex;
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
    .chapter-link {
        text-align: left;
        cursor: pointer;
        display: inline-flex;
        justify-content: left;
        flex-direction: column;
        padding-left: 10px;
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
        /* background-color: green; */
        border: 0;
        color: var(--text-color);
        font-size: large;
        height: inherit;
        margin: 0 2px;
    }
</style>
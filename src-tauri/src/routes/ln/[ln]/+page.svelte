<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { faEllipsisVertical, faBookmark, faAnglesDown, faCircleCheck, faSquareCheck, faCheck, faArrowTurnDown, faXmark } from '@fortawesome/free-solid-svg-icons'
    import { faCircleDown, faBookmark as faOutlineBookmark, faSquare } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa'
    import store from "$lib/store.js"
    import { find_ln } from "$lib/ln_common.js";
    import { Moon } from 'svelte-loading-spinners';
    import "$lib/css/listItems.css";

    export let data;

    let ln = {};
    let loading = false;

    // Adds to history when data is available
    $: if (Object.keys(ln).length != 0) {
        store.update(json => {
            json.ln_history.push(ln);
            return json;
        });
    }

    onMount(async () => {
        if (ln['chapters'].length == 0) {
            loading = true;
            let updated_ln = await invoke('get_ln_chapters', { ln });
            // let html = '';
            // if (c.extra.hasOwnProperty("request")) {
            //     if (c.extra.request == "post") {
            //         html = await invoke('post_fetch', {url: c.url});
            //     }
            // } else {
            //     html = await invoke('fetch', {url: c.url});
            // }
            // ln['chapters'] = eval(c.getChapters + `getChapters(${JSON.stringify(html)})`);
            updated_ln['chapters'].sort((a,b) => b.number-a.number);
            ln.author = updated_ln.author;
            ln.artist = updated_ln.artist;
            ln.description = updated_ln.description;
            ln.chapters = updated_ln.chapters;
            loading = false;
        }
    })
    
    store.subscribe(async (json) => {
        // gets ln search details
        ln = find_ln(data.id);
    });

    // CHAPTER OPTION BUTTONS
    function toggle_complete(index) {
        if (ln['chapters'][index].completed) {
            ln['chapters'][index].completed = false;
            ln['chapters'][index].page = 1;
        } else {
            ln['chapters'][index].completed = true;
        }
        invoke('update_ln_lib', { item: ln });
    }
    function check_below(index) {
        for (let i = index+1; i < ln['chapters'].length; i++) {
            ln['chapters'][i].completed = true;
        }
        invoke('update_ln_lib', { item: ln });
    }
    function remove_page(index) {
        ln['chapters'][index].page = 1;
        invoke('update_ln_lib', { item: ln });
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
        <img id="img" src="{ln.img}" alt="{ln.title}"/>
    </div>
    <div id="header-content">
        <div id="text">
            <h3>{ln.title}</h3>
            <p>Author: {ln.authors}</p>
            <p>Plugin: {ln.plugin}</p>
            <!-- TODO: fix desc scrolling -->
            <div id="desc"><p>{ln.description}</p></div>
        </div>
    </div>
</div>

<!-- Loading icon -->
<div style="margin: auto; width: fit-content; display: {loading? 'block' : 'none'}">
    <Moon color="var(--selection-color)" size="30" />
</div>

{#each ln['chapters'] as c, i}
<div class="chapter-item" style="{ln['chapters'][i].completed? 'color: grey' : ''}">
    <!-- Main Chapter button -->
    <button class="chapter-link" on:click={() => goto(`/ln/${data.id}/reader/${i}`)}>
        <p>
            {#if c.title == ""} Chapter {c.number}
            {:else} Chapter {c.number} - {c.title}
            {/if}
        </p>
        <div class="chapter-lower">
            <p>date - group</p>
            {#if ln['chapters'][i].page-1 != 0 && !ln['chapters'][i].completed}
                <p class="progress">&emsp;(page: {ln['chapters'][i].page})</p>
            {/if}
        </div>
    </button>
    <!-- options menu -->
    <div class="chap-btns">
        <!-- simple check icon -->
        <button id="check" class="chapter-btn" on:click={() => toggle_complete(i)}>
            {#if ln['chapters'][i].completed} <Fa icon={faXmark} />
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
            {#if ln['chapters'][i].page-1 != 0}
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
        --ln-img-width: 25vw;
    }
    #header {
        overflow: hidden;
        border-bottom: 1px solid black;
        display: inline-flex;
        height: calc(var(--ln-img-width) *1.5 + 20px);
        width: 100vw;
    }
    #header #img-wrapper {
        align-items: center;
    }
    #img-wrapper {
        width: var(--ln-img-width);
        height: calc(var(--ln-img-width) *1.5);
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
        width: calc(100% - var(--ln-img-width));
        padding-right: 10px;
        flex-direction: column;
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
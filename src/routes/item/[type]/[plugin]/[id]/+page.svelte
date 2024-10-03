<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { navigating } from '$app/stores';
    import { faEllipsisVertical, faBookmark, faAnglesDown, faCircleCheck, faSquareCheck, faCheck, faArrowTurnDown, faXmark } from '@fortawesome/free-solid-svg-icons'
    import { faCircleDown, faBookmark as faOutlineBookmark, faSquare } from '@fortawesome/free-regular-svg-icons';
    import Fa from 'svelte-fa'
    import store from "$lib/store.js"
    import { find_item } from "$lib/common.js";
    import { Moon } from 'svelte-loading-spinners';
    import "$lib/css/listItems.css";

    export let data;

    let item = {};
    let entry_type = data.type=="anime"? "episodes" : "chapters";
    let entry_text = data.type=="anime"? "Episode" : "Chapter";
    let loading = false;
    let error = "";

    onMount(async () => {
        loading = true;
        if (item[entry_type].length == 0) {
            let updated_item = await invoke(`get_${data.type}_${entry_type}`, { item });
            if (!updated_item.hasOwnProperty("error")) {
                if (data.type=="anime") {
                    item.studio = updated_item.studio;
                    item.status = updated_item.status;
                } else {
                    item.authors = updated_item.authors;
                    item.artists = updated_item.artists;
                }
                item.description = updated_item.description;
                item[entry_type] = updated_item[entry_type];
                // Updating databases
                store.update(async (json) => {
                    let toChange = json[`${data.type}_library`].find(i => i.id == data.id && i.plugin==data.plugin);
                    toChange = item;
                    await invoke(`update_${data.type}_lib`, { item });
                    return json;
                });
            } else {
                error = updated_item.error;
            }
        }
        // todo: sorting

        loading = false;
            item[entry_type] = item[entry_type].sort((a,b) => b.number-a.number);
    });
    
    store.subscribe(async (json) => {
        // gets item search details
        if (data.item != null) {
            item = data.item;
        } else {
            item = find_item(data.type, data.plugin, data.id);
        }
        console.log(item);
    });

    // ENTRY OPTION BUTTONS
    function toggle_complete(index) {
        if (item[entry_type][index].completed) {
            item[entry_type][index].completed = false;
            // todo: maybe this will cause errors one day~~
            item[entry_type][index].page = 1;
            item[entry_type][index].watch_time = 0;
        } else {
            item[entry_type][index].completed = true;
        }
        invoke(`update_${data.type}_lib`, { item });
    }
    function check_below(index) {
        for (let i = index+1; i < item[entry_type].length; i++) {
            item[entry_type][i].completed = true;
        }
        invoke(`update_${data.type}_lib`, { item });
    }
    function remove_page(index) {
        if (data.type == "anime") {
            item[entry_type][index].watch_time = 0;
        } else {
            item[entry_type][index].page = 1;
        }
        invoke(`update_${data.type}_lib`, { item });
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
        <img id="img" src="{item.img}" alt="{item.title}"/>
    </div>
    <div id="header-content">
        <div id="text">
            <h3>{item.title}</h3>
            {#if data.type == "anime"}
                <p>Status: {item.status}</p>
            {:else}
                <p>Author: {item.authors}</p>
            {/if}
            <p>Plugin: {item.plugin}</p>
            <!-- TODO: fix desc scrolling -->
            <div id="desc"><p>{item.description}</p></div>
        </div>
    </div>
</div>

<h3 class="entry-num">{item[entry_type].length} {entry_type}</h3>

<!-- Loading icon -->
<div style="margin: auto; width: fit-content; display: {loading? 'block' : 'none'}">
    <Moon color="var(--selection-color)" size="30" />
</div>

{#if error != ""}
    <p style="color: red; width: inherit; text-align: center; padding: 0; margin: 0">{error}</p>
{/if}

{#each item[entry_type] as c, i}
<div class="entry-item" style="{item[entry_type][i].completed? 'color: grey' : ''}">
    <!-- Main Entry button -->
    <button class="entry-link" on:click={() => goto(`/viewer/${data.type}/${data.plugin}/${data.id}/${i}`)}>
        <p>
            {#if c.title == "" || c.title.toLowerCase() == entry_text.toLowerCase()+" "+c.number} {entry_text} {c.number}
            {:else} {entry_text} {c.number} - {c.title}
            {/if}
        </p>
        <div class="entry-lower">
            <p>date - group</p>
            {#if item[entry_type][i].page-1 != 0 && !item[entry_type][i].completed && data.type != "anime"}
                <p class="progress">&emsp;(page: {item[entry_type][i].page})</p>
            {:else if item[entry_type][i].watch_time != 0 && !item[entry_type][i].completed && data.type == "anime"}
                <p class="progress">
                    &emsp;(progress: {parseInt(item[entry_type][i].watch_time/60)>0? parseInt(item[entry_type][i].watch_time/60) : 0}:{(parseInt(item[entry_type][i].watch_time)%60).toString().padStart(2, '0')}/{parseInt(item[entry_type][i].duration/60)}:{(parseInt(item[entry_type][i].duration)%60).toString().padStart(2, '0')})
                </p>
            {/if}
        </div>
    </button>
    <!-- options menu -->
    <div class="entry-btns">
        <!-- simple check icon -->
        <button id="check" class="entry-btn" on:click={() => toggle_complete(i)}>
            {#if item[entry_type][i].completed} <Fa icon={faXmark} />
            {:else} <Fa icon={faCheck} />
            {/if}
        </button>
        <!-- expanded options menu -->
        <div id="i-{i}" style="display:none">
            <!-- TODO: make work -->
            <button id="bookmark" class="entry-btn"><Fa icon={faOutlineBookmark} /></button>
            <button id="check-below" class="entry-btn" on:click={() => check_below(i)}><Fa icon={faAnglesDown} /></button>
            <button id="download" class="entry-btn"><Fa icon={faCircleDown} /><!--<Fa icon={faCircleCheck} />--></button>
            <button id="select" class="entry-btn"><Fa icon={faSquare} /><!--<Fa icon={faSquareCheck} />--></button>
            {#if (item[entry_type][i].page-1 != 0 && data.type != "anime") || (item[entry_type][i].watch_time != 0 && data.type == "anime")}
                <button id="uncheck" class="entry-btn" on:click={() => remove_page(i)}>
                    <Fa icon={faXmark} />
                </button>
            {/if}
            <button id="options-return" class="entry-btn" on:click={() => show_options(i)}>
                <Fa icon={faArrowTurnDown} />
            </button>
        </div>
        <!-- options expantion button -->
        <div id="o-{i}" style="display:flex">
            <button id="options" class="entry-btn" on:click={() => show_options(i)}>
                <Fa icon={faEllipsisVertical} />
            </button>
        </div>
    </div>
</div>
{/each}

<style>
    :root {
        --item-img-width: 25vw;
    }
    #header {
        --item-img-width: 25vw;
        overflow: hidden;
        border-bottom: 1px solid black;
        display: inline-flex;
        height: calc(var(--item-img-width) *1.5 + 20px);
        width: 100vw;
    }
    #header #img-wrapper {
        align-items: center;
    }
    #img-wrapper {
        width: var(--item-img-width);
        height: calc(var(--item-img-width) *1.5);
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
        width: calc(100% - var(--item-img-width));
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
    .entry-num {
        padding: 10px;
        margin: 0;
    }
    /* @media only screen and (max-width: 550px) {
        .entry-link p {
            font-size: medium;
        }
    } */
    .progress {
        color: grey;
    }
    .entry-btns {
        float: right;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        margin-right: 10px;
    }
    .entry-btn {
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

    .entry-lower {
        padding-top: 5px;
        display: inline-flex;
    }
</style>
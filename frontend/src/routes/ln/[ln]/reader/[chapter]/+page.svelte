<script>
    import { invoke } from "@tauri-apps/api/core";
    import { faArrowLeft, faBookmark, faEllipsisVertical } from '@fortawesome/free-solid-svg-icons';
    import { faBookmark as faOutlineBookmark } from '@fortawesome/free-regular-svg-icons';
    import { find_ln } from "$lib/ln_common.js";
    import { goto } from "$app/navigation";
    import { Moon } from 'svelte-loading-spinners';
    import Fa from 'svelte-fa'
    export let data;

    let ln = find_ln(data.id);
    let chapter = ln['chapters'][data.ln_index];
    let curr_page = 0;
    let display_page = 0;
    let imgs = -1;
    let paragraphs = [];
    let is_loading = false;

    // Starts when page starts
    set_colors();
    start_reader(chapter.page-1);
    
    // Prepares Reader & pages
    async function start_reader(page) {
        is_loading = true;
        paragraphs = [];
        chapter = ln['chapters'][data.ln_index];
        imgs = document.getElementsByTagName("img");
        let c = await invoke('get_ln_pages', { source: ln.plugin, id: chapter.id });
        let html = await invoke('fetch', {url: c.url});
        paragraphs = eval(c.getChapterPages + `getChapterPages(${JSON.stringify(html)})`);
        // chapters = await getChapterPages(ln.extention, chapter.id);
        // if (chapter.completed && page != Infinity) {
        //     curr_page = 0;
        // } else if (page == Infinity) {
        //     curr_page = chapters.length-1;
        // } else {
        //     curr_page = page;
        // }
        is_loading = false;
    }

    function set_colors() {
        // Sets a background color to have more transparency
        let secondaryColor = getComputedStyle(document.body).getPropertyValue('--secondary-color').trim();
        let hex = parseInt(secondaryColor.slice(1), 16);
        let [r, g, b] = [(hex >> 16) & 255, (hex >> 8) & 255, hex & 255];
        document.documentElement.style.setProperty('--secondary-color-transparent', `rgba(${r}, ${g}, ${b}, 0.5)`);
        // Sets text outline to opposite of current
        let primaryColor = getComputedStyle(document.body).getPropertyValue('--primary-color').trim();
        if (primaryColor == "#f2f2f2") {
            document.documentElement.style.setProperty('--text-outline', `1px 0 0 #fff, 0 -1px 0 #fff, 0 1px 0 #fff, -1px 0 0 #fff`);
        } else if (primaryColor == "#1a1a1a") {
            document.documentElement.style.setProperty('--text-outline', `1px 0 0 #000, 0 -1px 0 #000, 0 1px 0 #000, -1px 0 0 #000`);
        } else {
            document.documentElement.style.setProperty('--text-outline', `0`);
        }
    }

    // Update display page number, unless it is above or below a threshold
    // $: if (curr_page >= 0 && curr_page < chapters.length) display_page = curr_page+1;
    
    // Updates library backend
    async function update_lib() {
        // if (curr_page >= imgs.length-1) chapter.completed = true;
        await invoke('update_ln_lib', { item: ln });
        goto(`/ln/${data.id}`)
    }

    // ----- INPUT -----
    // window.addEventListener('keydown', keyInput);
    function keyInput(event) {
        switch (event.key) {
            case "ArrowRight":
            case 'd':
            case " ":
                next();
                break;
            case "ArrowLeft":
            case 'a':
                prev();
                break;
        }
    }
    function toggle_menu() {
        if (document.getElementById("chap-menu").style.opacity == "1") {
            document.getElementById("chap-menu").style.opacity = "0";
        } else {
            document.getElementById("chap-menu").style.opacity = "1";
        }
    }
    function next() {
    //     if (curr_page < imgs.length-1) {
    //         curr_page++;
    //         chapter.page = curr_page+1;
    //         adjustImage();
    //     } else if (curr_page == imgs.length-1) {
    //         curr_page++;
    //         adjustImage();
    //     } else {
    //         let next = parseInt(data.ln_index)-1;
    //         if (next >= 0) {
    //             goto(`/ln/${data.id}/reader/${next}`).then(() => {
    //                 chapter.completed = true;
    //                 invoke('update_ln_lib', { item: ln }).then(() => {
    //                     start_reader(0);
    //                 });
    //             });
    //         } else {
    //             update_lib();
    //         }
    //     }
    }
    function prev() {
    //     if (curr_page > 0) {
    //         curr_page--;
    //         chapter.page = curr_page+1;
    //         adjustImage();
    //     } else if (curr_page == 0) {
    //         curr_page--;
    //     } else {
    //         let prev = parseInt(data.ln_index)+1;
    //         if (prev < ln['chapters'].length) {
    //             goto(`/ln/${data.id}/reader/${prev}`).then(() => {
    //                 invoke('update_ln_lib', { item: ln }).then(() => {
    //                     start_reader(Infinity);
    //                 });
    //             });
    //         } else {
    //             update_lib()
    //         }
    //     }
    }

</script>

<svelte:window on:keydown={keyInput} />
<!-- SNACKBAR -->
<div id="chap-menu" style="opacity: 0">
    <div class="menu-background"></div>
    <div id="chap-snackbar">
        <button class="chap-snack-item" on:click={update_lib}><Fa icon={faArrowLeft} /></button>
        <div id="chap-snack-text">
            <p>{ln.title}</p>
            <p style="font-size: x-small">
                {#if chapter.title == ""}
                    Chapter {chapter.number}
                {:else}
                    Chapter {chapter.number}: {chapter.title}
                {/if}
            </p>
        </div>
        <div class="chap-snack-right">
            <button class="chap-snack-item"><Fa icon={faOutlineBookmark} /></button>
            <button class="chap-snack-item"><Fa icon={faEllipsisVertical} /></button>
        </div>
    </div>
    
    <!-- ARROWS -->
    <button id="prev" class="arrow" on:click={prev}>&lt;</button>
    <button id="show-arrow" on:click={toggle_menu}></button>
    <button id="next" class="arrow" on:click={next}>&gt;</button>
    
    <!-- FOOTER -->
    <div id="page-num">
        <p>
            -/-
        </p>
    </div>
</div>

<!-- PAGES -->
<div class="content">
    <div class="loading" style="display: {is_loading? 'flex' : 'none'}">
        <Moon color="var(--selection-color)" size="30" />
    </div>
    {#each paragraphs as p, i}
        <p>{p}</p>
    {/each}
</div>

<style>
    /* content */
    .content {
        margin: 0 20px;
    }
    .loading {
        display: flex;
        width: 100vw;
        height: 100vh;
        justify-content: center;
        align-items: center;
    }
    /* arrows */
    #chap-menu {
        position: absolute;
        width: 100vw;
    }
    #chap-snackbar {
        /* opacity: 0.5; */
        height: calc(var(--snackbar-height)*1.4);
        /* background-color: var(--secondary-color); */
        position: absolute;
        width: 100vw;
    }
    .menu-background {
        width: 100vw;
        position: absolute;
        opacity: 0.5;
        height: calc(var(--snackbar-height)*1.4);
        background-color: var(--secondary-color);
    }
    .chap-snack-item {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        background-color: transparent;
        /* background-color: green; */
        border: 0;
        color: var(--text-color);
        font-size: medium;
        height: inherit;
        width: 50px;
    }
    button.chap-snack-item:hover {
        background-color: var(--selection-color);
        border-radius: 50%;
    }
    #chap-snack-text {
        position: relative;
        top: -9px;
        display: inline-flex; 
        height: inherit; 
        flex-direction: column;
        width: calc(100vw - 3*53px);
    }
    #chap-snack-text p {
        padding: 0;
        margin: 0;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
    }
    .chap-snack-right {
        float: right; 
        height: inherit
    }
    #show-arrow {
        position: relative;
        height: 100vh;
        width: 30vw;
        opacity: 50%;
        margin: auto;
        opacity: 0%;
        height: calc(100vh - calc(var(--snackbar-height)*2));
        top: calc(var(--snackbar-height)*2);
    }
    .arrow {
        opacity: 0;
        position: relative;
        height: calc(100vh - calc(var(--snackbar-height)*2));
        top: calc(var(--snackbar-height)*2);
        width: 35vw;
        font-size: xx-large;
    }
    #prev {
        float: left;
    }
    #next {
        float: right;
    }
    #page-num {
        position: absolute;
        width: 100vw;
        text-align: center;
        /* bottom: -15px; */
        font-weight: 600;
        text-shadow: var(--text-outline);
    }
    #page-num p {
        padding: 10px;
        margin: 0;
        background-color: var(--secondary-color-transparent);
        border-radius: 30%;
        width: fit-content;
        display: inline-flex;
        justify-content: center;
    }
</style>
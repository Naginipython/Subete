<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { faArrowLeft, faBookmark } from '@fortawesome/free-solid-svg-icons';
    import { faBookmark as faOutlineBookmark } from '@fortawesome/free-regular-svg-icons';
    // import { getChapterPages } from "$lib/manga_sources/main.js"
    import { find_manga } from "$lib/common.js";
    import { goto } from "$app/navigation";
    import Fa from 'svelte-fa'
    export let data;

    let manga = find_manga(data.id);
    let chapter = manga['chapters'][data.manga_index];
    let curr_page = 0;
    let display_page = 0;
    let imgs = -1;
    let chapters = [];

    start_reader(chapter.page-1);
    
    // Prepares Reader & pages
    async function start_reader(page) {
        chapters = [];
        chapter = manga['chapters'][data.manga_index];
        imgs = document.getElementsByTagName("img");
        chapters = await invoke('get_pages', { source: manga.extension, id: chapter.id });
        // chapters = await getChapterPages(manga.extention, chapter.id);
        if (chapter.completed && page != Infinity) {
            curr_page = 0;
        } else if (page == Infinity) {
            curr_page = chapters.length-1;
        } else {
            curr_page = page;
        }
    }

    // Checks for completion
    $: if (imgs.length > 0 && !chapter.completed) {
        if (curr_page >= imgs.length-1) chapter.completed = true;
        else chapter.completed = false;
    }

    // Update display page number, unless it is above or below a threshold
    $: if (curr_page >= 0 && curr_page < chapters.length) display_page = curr_page+1;
    
    // Updates library backend
    async function update_lib() {
        console.log(chapter.completed);
        console.log(chapter.page);
        await invoke('update_lib', { item: manga });
        goto(`/manga/${data.id}`)
    }

    // ----- INPUT -----
    window.addEventListener('keydown', keyInput);
    function keyInput(key) {
        switch (key.keyCode) {
            case 39: // left
            case 'd': // d
            case 32: // space
                next();
                break;
            case 37: //right
            case 'a': // a
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
        if (curr_page < imgs.length-1) {
            curr_page++;
            chapter.page = curr_page+1;
            adjustImage();
        } else if (curr_page == imgs.length-1) {
            curr_page++;
            adjustImage();
        } else {
            let next = parseInt(data.manga_index)-1;
            if (next >= 0) {
                goto(`/manga/${data.id}/reader/${next}`).then(() => {
                    invoke('update_lib', { item: manga }).then(() => {
                        start_reader(0);
                    });
                });
            } else {
                update_lib();
            }
        }
    }
    function prev() {
        if (curr_page > 0) {
            curr_page--;
            chapter.page = curr_page+1;
            adjustImage();
        } else if (curr_page == 0) {
            curr_page--;
        } else {
            let prev = parseInt(data.manga_index)+1;
            if (prev < manga['chapters'].length) {
                goto(`/manga/${data.id}/reader/${prev}`).then(() => {
                    start_reader(Infinity);
                    // invoke('update_lib', { item: manga }).then(() => {
                    // });
                });
            } else {
                update_lib()
            }
        }
    }

    // Image Fitting
    window.addEventListener('resize', adjustImage);
    // todo: fix this
    function adjustImage() {
        // console.log("adjustImage start")
        if (imgs[curr_page] != undefined) {
            // Inital sizer
            if (imgs[curr_page].height > imgs[curr_page].width) {
            // console.log("height: 100vw");
                imgs[curr_page].style.height = "100vh";
                imgs[curr_page].style.width = "auto";
            } else {
            // console.log("width: 100vw");
                imgs[curr_page].style.height = "auto";
                imgs[curr_page].style.width = "100vw";
            }
            // double checking size compared to viewport
            // let height_diff = imgs[curr_page].height - window.innerHeight;
            // if (height_diff > 0) {
            // console.log("got here3");
            //     imgs[curr_page].style.height = "auto";
            //     imgs[curr_page].style.width = "100vw";
            // } 
            // let width_dif = imgs[curr_page].width - window.innerWidth
            // if (width_dif > 0) {
            // console.log("got here4");
            //     imgs[curr_page].style.height = "100vh";
            //     imgs[curr_page].style.width = "auto";
            // }
        }
    }
</script>
<!-- <svelte:window on:keydown|preventDefault={keyInput} /> -->
<div>
    
    <div id="chap-menu" style="opacity: 0">
        <div class="menu-background"></div>
        <div id="chap-snackbar">
            <button class="chap-snack-item" on:click={update_lib}><Fa icon={faArrowLeft} /></button>
            <div id="chap-snack-text">
                <p>{manga.title}</p>
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
            </div>
        </div>
        
        <button id="prev" class="arrow" on:click={prev}>&lt;</button>
        <button id="show-arrow" on:click={toggle_menu}></button>
        <button id="next" class="arrow" on:click={next}>&gt;</button>
        
        <div id="page-num">
            <p>
                {display_page}/{imgs.length}
            </p>
        </div>
    </div>
    <div class="center-img-div">
        <p id="prev-chapter" class={curr_page == -1? 'visible' : 'invisible'}>previous chapter?</p>
        {#if chapter.length == 0}
            <p style="color: white; font-size: xx-large">loading</p>
        {:else}
            {#each chapters as c, i}
                <img on:load={adjustImage} class={i == curr_page? 'visible' : 'invisible'} src={c} alt="{i} - {manga.title}" />
            {/each}
        {/if}
            <p id="next-chapter" class={curr_page == chapter.page? 'visible' : 'invisible'}>next chapter?</p>
    </div>
</div>

<style>
    /* img */
    .visible {
        display: flex;
        margin: auto;
        justify-content: center;
    }
    .invisible {
        display: none;
    }
    .center-img-div {
        display: flex; 
        justify-content: center; 
        height: 100vh;
        overflow: hidden;
    }
    .center-img-div img {
        /* Purely to look better when it pops in */
        height: 0px;
    }
    /* arrows */
    #chap-menu {
        position: absolute;
        width: 100vw;
    }
    #chap-snackbar {
        /* opacity: 0.5; */
        height: calc(var(--snackbar-height)*2);
        /* background-color: var(--secondary-color); */
        position: absolute;
        width: 100vw;
    }
    .menu-background {
        width: 100vw;
        position: absolute;
        opacity: 0.5;
        height: calc(var(--snackbar-height)*2);
        background-color: var(--secondary-color);
    }
    .chap-snack-item {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        background-color: transparent;
        /* background-color: green; */
        border: 0;
        color: white;
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
        width: calc(100vw - 2*52px);
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
        text-shadow: 1px 0 0 #000, 0 -1px 0 #000, 0 1px 0 #000, -1px 0 0 #000;
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
<script>
    import store from "$lib/store.js"
    import { onMount } from "svelte";
    import { invoke } from '@tauri-apps/api/tauri';
    import { getChapterPages } from "$lib/manga_sources/main.js"
    export let data;

    let manga = {};
    let index = -1;
    let page = 0;
    let imgs = -1;
    let chapters = []

    onMount(async () => {
        index = data.manga_index;
        imgs = document.getElementsByTagName("img");
    });

    store.subscribe(async (json) => {
        let manga_test = json.library.find(m => m.id == data.id);
        if (manga_test == undefined) {
            let search_test = json.search_results.find(m => m.id == data.id);
            if (search_test == undefined) {
                manga = json.history.find(m => m.id == data.id);
            } else {
                manga = search_test;
            }
        } else {
            manga = manga_test;
        }
        
        index = manga['chapters'][data.manga_index].id;
        page = manga['chapters'][data.manga_index].page-1;
        chapters = await getChapterPages(manga.extention, index);
        return json;
    });

    function next() {
        if (page < imgs.length-1) {
            imgs[page].classList.remove('visible');
            imgs[page++].classList.add('invisible');
            imgs[page].classList.add("visible");
            imgs[page].classList.remove("invisible");
            manga['chapters'][data.manga_index].page = page+1;
            adjustImage();
        }
    }
    function prev() {
        if (page > 0) {
            imgs[page].classList.remove('visible');
            imgs[page--].classList.add('invisible');
            imgs[page].classList.add("visible");
            imgs[page].classList.remove("invisible");
            manga['chapters'][data.manga_index].page = page+1;
        }
    }
    async function update_lib() {
        await invoke('update_lib', { item: manga });
        // store.update(json => {
        //     json.library.push(manga);
        //     return json;
        // });
    }
    function toggle_arrows() {
        let to_toggle = ["next", "prev", "return", "page-num"]
        if (document.getElementById("next").style.opacity == "0.5") {
            to_toggle.forEach(e => document.getElementById(e).style.opacity = "0");
        } else {
            to_toggle.forEach(e => document.getElementById(e).style.opacity = "0.5");
            // exception
            document.getElementById("page-num").style.opacity = "1"
        }
    }
    
    function keyInput(key) {
        switch (key.keyCode) {
            case 39: // left
            case 68: // d
            case 32: // space
                next();
                break;
            case 37: //right
            case 65: // a
                prev();
                break;
        }
    }

    window.addEventListener('resize', adjustImage);
    function adjustImage() {
        if (imgs[page] != undefined) {
            // Inital sizer
            if (imgs[page].height > imgs[page].width) {
                imgs[page].style.height = "100vh";
                imgs[page].style.width = "auto";
            } else {
                imgs[page].style.height = "auto";
                imgs[page].style.width = "100vw";
            }
            // double checking size compared to viewport
            let width_dif = imgs[page].width - window.innerWidth
            let height_diff = imgs[page].height - window.innerHeight;
            if (width_dif > 0) {
                imgs[page].style.height = "auto";
                imgs[page].style.width = "100vw";
            } 
            if (height_diff > 0) {
                imgs[page].style.height = "100vh";
                imgs[page].style.width = "auto";
            }
        }
    }
</script>
<svelte:window on:keydown|preventDefault={keyInput} />
<div>
    <div id="arrow_div">
        <button id="prev" class="arrow" on:click={prev} style="opacity: 0%">&lt;</button>
        <button id="show-arrow" on:click={toggle_arrows}></button>
        <button id="next" class="arrow" on:click={next} style="opacity: 0%">&gt;</button>
        <a id="return" href='/manga/{data.id}' style="opacity: 0%" on:click={update_lib}>Return</a>
    </div>
    <div class="center-img-div">
        {#each chapters as c, i}
            {#if i == manga['chapters'][data.manga_index].page-1}
                <img on:load={adjustImage} class='visible' src={c} alt="{i} - {manga.title}" />
            {:else}
                <img class='invisible' src={c} alt={c} />
            {/if}
        {/each}
        <p id="next-chapter" class='invisible'>next chapter?</p>
    </div>
    <p id="page-num" style="opacity: 0%">
        {page+1}/{imgs.length}
    </p>
</div>

<style>
    /* remove body shit */
    :global(body) {
        margin: 0;
    }
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
    #arrow_div {
        position: absolute;
        width: 100vw;
    }
    #show-arrow {
        position: relative;
        height: 100vh;
        width: 30vw;
        opacity: 50%;
        margin: auto;
        opacity: 0%;
    }
    .arrow {
        position: relative;
        height: 100vh;
        width: 35vw;
        font-size: xx-large;
    }
    #prev {
        float: left;
    }
    #next {
        float: right;
    }
    #return {
        position: absolute;
        top: 0;
        left: 0;
        font-size: large;
        width: 35vw;
        text-decoration: none;
        align-items: flex-start;
        text-align: center;
        cursor: default;
        color: buttontext;
        padding-block-start: 2px;
        padding-block-end: 3px;
        /* padding-inline-start: 6px; */
        border-top-width: 2px;
        border-right-width: 2px;
        border-bottom-width: 2px;
        border-left-width: 2px;
        border-top-style: outset;
        border-right-style: outset;
        border-bottom-style: outset;
        border-left-style: outset;
        border-top-color: buttonface;
        border-right-color: buttonface;
        border-bottom-color: buttonface;
        border-left-color: buttonface;
        /* background-color: buttonface; */
        box-sizing: border-box;
    }
    #next-chapter {
        position: absolute;
        top: 40vh;
        width: 100vw;
        justify-content: center;
        text-align: center;

    }
    #page-num {
        position: absolute;
        bottom: -15px;
        left: 50%;
        right: 50%;
        font-weight: 600;
        text-shadow: 1px 0 0 #000, 0 -1px 0 #000, 0 1px 0 #000, -1px 0 0 #000;
    }
</style>
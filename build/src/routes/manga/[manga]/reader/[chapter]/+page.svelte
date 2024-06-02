<script>
    import store from "$lib/store.js"
    import { invoke } from '@tauri-apps/api/tauri';
    import { getChapterPages } from "$lib/manga_sources/main.js"
    import { goto } from "$app/navigation";
    export let data;

    let manga = {};
    let curr_page = 0;
    let imgs = -1;
    let chapters = [];
    
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
        start_reader(manga['chapters'][data.manga_index].page-1);
        
        return json;
    });
    async function start_reader(page) {
        chapters = [];
        imgs = document.getElementsByTagName("img");
        chapters = await getChapterPages(manga.extention, manga['chapters'][data.manga_index].id);
        if (manga['chapters'][data.manga_index].completed && page != Infinity) {
            console.log("got here")
            curr_page = 0;
        } else if (page == Infinity) {
            curr_page = chapters.length-1;
        } else {
            curr_page = page;
        }
    }

    $: if (imgs.length > 0) {
        if (curr_page >= imgs.length-1) manga['chapters'][data.manga_index].completed = true;
        else manga['chapters'][data.manga_index].completed = false;
    }
    
    async function update_lib() {
        await invoke('update_lib', { item: manga });
    }

    // ----- INPUT -----
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
    function next() {
        if (curr_page < imgs.length-1) {
            curr_page++;
            manga['chapters'][data.manga_index].page = curr_page+1;
            adjustImage();
        } else if (curr_page == imgs.length-1) {
            curr_page++;
            adjustImage();
        } else {
            goto(`/manga/${data.id}/reader/${parseInt(data.manga_index)+1}`).then(() => {
                update_lib();
                start_reader(0);
            });
        }
    }
    function prev() {
        if (curr_page > 0) {
            curr_page--;
            manga['chapters'][data.manga_index].page = curr_page+1;
            adjustImage();
        } else if (curr_page == 0) {
            curr_page--;
        } else {
            goto(`/manga/${data.id}/reader/${parseInt(data.manga_index)-1}`).then(() => {
                update_lib();
                start_reader(Infinity);
            });
        }
    }
    window.addEventListener('resize', adjustImage);
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
<svelte:window on:keydown|preventDefault={keyInput} />
<div>
    <div id="arrow_div">
        <button id="prev" class="arrow" on:click={prev} style="opacity: 0%">&lt;</button>
        <button id="show-arrow" on:click={toggle_arrows}></button>
        <button id="next" class="arrow" on:click={next} style="opacity: 0%">&gt;</button>
        <a id="return" href="/manga/{data.id}" style="opacity: 0%" on:click={update_lib}>Return</a>
    </div>
    <div class="center-img-div">
        <p id="prev-chapter" class={curr_page == -1? 'visible' : 'invisible'}>previous chapter?</p>
        {#each chapters as c, i}
            <img on:load={adjustImage} class={i == curr_page? 'visible' : 'invisible'} src={c} alt="{i} - {manga.title}" />
        {/each}
            <p id="next-chapter" class={curr_page == manga['chapters'][data.manga_index].page? 'visible' : 'invisible'}>next chapter?</p>
    </div>
    <p id="page-num" style="opacity: 0%">
        {curr_page+1}/{imgs.length}
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
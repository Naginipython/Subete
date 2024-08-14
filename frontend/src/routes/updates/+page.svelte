<script>
    import store from "$lib/store.js";
    import { goto } from "$app/navigation";
    import { find_chapter_index_by_id } from "$lib/manga_common.js";

    let media_screen = "manga";
    let update = [];
    let progress = null;//{curr: "test",index:1,total:5}//

    store.subscribe(json => {
        progress = json.manga_update_progress;

        // Sets limit on Update size
        while (json['manga_updates'] > 100) {
            json['manga_updates'].pop();
        }
        while (json['ln_updates'] > 100) {
            json['anime_updates'].pop();
        }
        // while (json['manga_updates'] > 100) {
        //     json['manga_updates'].pop();
        // }

        media_screen = json['media_screen'];
        switch (media_screen) {
            case "manga": 
                media_screen = "Manga";
                update = json["manga_updates"];
                break;
            case "anime": 
                media_screen = "Anime";
                update = [];
                break;
            case "ln": 
                media_screen = "Light Novel";
                update = json["ln_updates"];
                break;
        }
    });

    async function goToChapter(u) {
        let index = find_chapter_index_by_id(u.id, u.chapter.id);
        goto(`/manga/${u.id}/reader/${index}`);
    }
</script>
<div id="update-header">
    <h3 >{media_screen} Updates</h3>
</div>

{#if progress != null}
    <div id="update-progress-bar">
        <p>Current: {progress.curr} ({progress.index}/{progress.total})</p>
        <progress value={progress.index} max={progress.total} /> 
    </div>
{/if}

{#each update as u, i}
    <!-- style="{manga['chapters'][i].completed? 'color: grey' : ''}" -->
    <div class="update-chapter">
        <button class="update-manga-wrap" on:click={async () => goto(`/manga/${u.id}`)}>
            <img class="update-img" src={u.img} alt="title">
        </button>
        <button class="update-link" on:click={async () => {await goToChapter(u)}}>
            <p>
                {u.title}
            </p>
            <div class="update-lower">
                {#if u.chapter.title == ""} Chapter {u.chapter.number}
                {:else} Chapter {u.chapter.number} - {u.chapter.title}
                {/if}
                <!-- {#if u.manga['chapters'][i].page-1 != 0 && !u.manga['chapters'][i].completed} -->
                    <!-- <p class="progress">&emsp;(page: {u.manga['chapters'][i].page})</p> -->
                <!-- {/if} -->
            </div>
        </button>
    </div>
{/each}

<style>
    #update-header {
        width: 100vw;
        display: inline-flex;
        justify-content: space-between;
        align-items: center;
    }
    #update-header h3 {
        margin-left: 10px;
    }
    #update-progress-bar {
        display: flex;
        flex-direction: column;
        align-items: left;
        width: calc(100vw - 20px);
        padding: 0 10px;
        padding-bottom: 10px;
    }
    #update-progress-bar progress {
        width: inherit;
    }
    #update-progress-bar p {
        margin: 0;
        padding: 0;
    }
    .update-chapter {
        width: 100vw;
        padding: 5px 0;
        overflow: auto;
        display: inline-flex;
        background-color: var(--primary-color);
        filter: brightness(0.95);
        align-items: center;
    }
    .update-chapter:hover {
        background-color: var(--selection-color)
    }
    .update-manga-wrap {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 50px;
        height: 50px;
        border-radius: 5px;
        overflow: hidden;
        border: 0;
        background-color: transparent;
    }
    .update-img {
        /* height: 50px; */
        /* width: auto; */
        height: 120%;
        width: auto;
    }
    .update-link p {
        justify-content: left;
        width: inherit;
        padding: 0;
        margin: 0;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
    }
    .update-link {
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
        height:fit-content;
        
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        /* width: idk; TODO */
    }
    .update-lower {
        display: inline-flex;
    }
    /* .update-lower p {
        margin: 0; 
        padding: 0; 
        font-size: x-small;
    } */
</style>
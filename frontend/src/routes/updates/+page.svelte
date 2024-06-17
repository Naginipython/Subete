<script>
    import store from "$lib/store.js";

    let media_screen = "manga";
    let update = [];

    store.subscribe(json => {
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
</script>

<h3 class="update-header">{media_screen} Updates</h3>

{#each update as u, i}
    <!-- style="{manga['chapters'][i].completed? 'color: grey' : ''}" -->
    <div class="update-chapter">
        <img class="update-img" src="https://gogocdn.net/cover/akame-ga-kill-dub.png" alt="title">
        <button class="update-link" on:click={() => console.log(`/manga/manga_id/reader/i`)}>
            <p>
                temp
                <!-- {#if u.chapter.title == ""} Chapter {u.chapter.number} -->
                <!-- {:else} Chapter {u.chapter.number} - {u.chapter.title} -->
                <!-- {/if} -->
            </p>
            <div class="update-lower">
                <p>date - group</p>
                <!-- {#if u.manga['chapters'][i].page-1 != 0 && !u.manga['chapters'][i].completed} -->
                    <!-- <p class="progress">&emsp;(page: {u.manga['chapters'][i].page})</p> -->
                <!-- {/if} -->
            </div>
        </button>
    </div>
{/each}

<style>
    .update-header {
        margin-left: 10px;
    }
    .update-chapter {
        width: 100vw;
        padding: 5px 0;
        overflow: auto;
        display: inline-flex;
        background-color: var(--primary-color);
        filter: brightness(0.95)
    }
    .update-chapter:hover {
        background-color: var(--selection-color)
    }
    .update-img {
        height: 50px;
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
        float: left;
        
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        /* width: idk; TODO */
    }
    .update-lower {
        display: inline-flex;
    }
    .update-lower p {
        margin: 0; 
        padding: 0; 
        font-size: x-small;
    }
</style>
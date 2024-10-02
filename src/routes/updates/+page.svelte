<script>
    import store from "$lib/store.js";
    import { goto } from "$app/navigation";
    import { find_entry_index_by_id } from "$lib/common.js";
    import "$lib/css/listItems.css";

    let media_screen = "manga";
    let header = "Manga"
    let update = [];
    let progress = null;//{curr: "test",index:1,total:5}//

    store.subscribe(json => {
        media_screen = json['media_screen'];
        console.log(json['media_screen'])
        switch (media_screen) {
            case "manga":
                header = "Manga";
                update = json["manga_updates"];
                progress = json.manga_update_progress;
                break;
            case "anime":
                header = "Anime";
                update = json["anime_updates"];
                progress = json.anime_update_progress;
                break;
            case "ln":
                header = "Light Novel"
                update = json["ln_updates"];
                progress = json.ln_update_progress;
                break;
        }
        // Sets limit on Update size
        while (json['manga_updates'] > 100) {
            json['manga_updates'].pop();
        }
        while (json['ln_updates'] > 100) {
            json['anime_updates'].pop();
        }
        while (json['anime_updates'] > 100) {
            json['anime_updates'].pop();
        }
    });

    async function goToEntry(u) {
        let index = find_entry_index_by_id(media_screen, u.id, u.chapter.id);
        goto(`/viewer/${media_screen}/${u.plugin}/${u.id}/${index}`);
    }
</script>
<div id="update-header">
    <h3 >{header} Updates</h3>
</div>

{#if progress != null}
    <div id="update-progress-bar">
        <p>Current: {progress.curr} ({progress.index}/{progress.total})</p>
        <progress value={progress.index} max={progress.total} /> 
    </div>
{/if}

{#each update as u, i}
    <!-- style="{manga['chapters'][i].completed? 'color: grey' : ''}" -->
    <div class="entry-item">
        <button class="entry-left" on:click={async () => goto(`/${media_screen}/${u.plugin}/${u.id}`)}>
            <img class="entry-left-img" src={u.img} alt="title">
        </button>
        <button class="entry-link" on:click={async () => {await goToEntry(u)}}>
            <p>
                {u.title}
            </p>
            <div class="entry-link-lower">
                {#if media_screen.toLowerCase() == "anime"}
                    {#if u.episode.title == ""} Episode {u.episode.number}
                    {:else} Episode {u.episode.number} - {u.episode.title}
                    {/if}
                {:else}
                    {#if u.chapter.title == ""} Chapter {u.chapter.number}
                    {:else} Chapter {u.chapter.number} - {u.chapter.title}
                    {/if}
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
</style>
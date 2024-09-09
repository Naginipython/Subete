<script>
    import store from "$lib/store.js";
    import "$lib/css/listItems.css";

    let manga_history = [];
    store.subscribe(json => {
        manga_history = json.manga_history;
    });
</script>

{#each manga_history as m}
    <div class="chapter-item">
        <button class="chapter-manga">
            <img class="chapter-manga-img" src={m.img} alt="title">
        </button>
        <button class="chapter-link">
            <p>{m.title}</p>
            <div class="chapter-link-lower">
                chp. {m.recent_chapter_num} - 12:00 AM
            </div>
        </button>
    </div>
{/each}
<style>
    :global(body) {
        margin: 0;
    }
    #lib-manga-section {
        display: flex;
        flex-wrap: wrap;
        justify-content: flex-start;
        overflow-y: auto;
        margin: auto;
        width: var(--calculated-width);
    }
    .manga {
        text-align: center;
        width: fit-content;
    }
    .manga-wrap {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 200px;
        height: 300px;
        margin: 5px;
        border-radius: 5px;
        overflow: hidden;
    }
    .manga-wrap img {
        width: auto;
        height: 105%;
    }
</style>

<script>
    import store from "$lib/store.js";

    let library = [];

    store.subscribe(json => {
        library = json["library"];
        return library;
    });

    window.addEventListener('resize', setWidth);
    setWidth();

    function setWidth() {
        var viewportWidth = window.innerWidth;
        var newWidth = Math.floor(viewportWidth / 210) * 220;
        
        document.documentElement.style.setProperty('--calculated-width', `${newWidth}px`);
    }
</script>

<div id="lib-manga-section">
{#each library as l, i}
    <a class="manga" href="/manga/{l.id}">
        <div class="manga-wrap">
            <img class="temp" src={l.img} alt={l.title}/>
            <!-- <p>{l.title}</p> -->
        </div>
    </a>
{/each}
</div>
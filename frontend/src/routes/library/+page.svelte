<style>
    :root {
        --lib-manga-height: 300px;
    }
    #lib-manga-section {
        display: flex;
        flex-wrap: wrap;
        justify-content: flex-start;
        overflow-y: auto;
        margin: auto;
        width: var(--calculated-width);
    }
    .lib-manga {
        text-align: center;
        width: fit-content;
        text-decoration: none;
        color: white;
        margin: 5px;
    }
    .lib-manga-wrap {
        display: flex;
        justify-content: center;
        align-items: center;
        width: calc(0.5*var(--lib-manga-height) + 50px);
        height: var(--lib-manga-height);
        border-radius: 5px;
        overflow: hidden;
    }
    .lib-manga-wrap img {
        width: auto;
        height: 105%;
    }
    .lib-manga p {
        margin: 0;
        padding: 0;
        width: calc(0.5*var(--lib-manga-height) + 50px);
        height: 1.5em;
        font-size: x-small;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        line-clamp: 2;
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
    <a class="lib-manga" href="/manga/{l.id}">
        <div class="lib-manga-wrap">
            <img src={l.img} alt={l.title}/>
        </div>
        <p>{l.title}</p>
    </a>
{/each}
</div>
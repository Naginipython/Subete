<script>
    import store from "$lib/store.js";

    let library = [];

    store.subscribe(json => {
        library = json["library"];
        setWidth();
        return library;
    });

    window.addEventListener('resize', setWidth);

    function setWidth() {
        var style = parseFloat(getComputedStyle(document.body).getPropertyValue('--lib-manga-height'));
        let imgWidth = 0.5*parseFloat(style) + 50 + 10; // +5 * itemcount
        let itemCount = Math.floor(window.innerWidth / imgWidth);
        var newWidth = itemCount * imgWidth;
        document.documentElement.style.setProperty('--calculated-width', `${newWidth}px`);
    }
</script>

<!-- TODO: categories -->
<div id="lib-manga-section">
<!-- TODO: loading icon -->
{#each library as l, i}
    <a class="lib-manga" href="/manga/{l.id}">
        <div class="lib-manga-wrap">
            <img src={l.img} alt={l.title}/>
        </div>
        <p>{l.title}</p>
    </a>
{/each}
</div>

<style>
    :root {
        --lib-manga-height: 300px;
        --lib-manga-width: calc(0.5*var(--lib-manga-height) + 50px);
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
        width: var(--lib-manga-width);
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
        width: var(--lib-manga-width);
        height: 1.5em;
        font-size: x-small;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        line-clamp: 2;
    }
</style>
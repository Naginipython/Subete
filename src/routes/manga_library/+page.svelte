<script>
    import store from "$lib/store.js";

    let library = [];
    let width = document.documentElement.style.getPropertyValue('--lib-manga-width');

    store.subscribe(json => {
        library = json["manga_library"];
        if (json["settings"].hasOwnProperty("library_widths")) {
            if (json["settings"]["library_widths"].hasOwnProperty("manga")) {
                width = json["settings"]["library_widths"].manga;
            }
        }
        setWidth();
    });

    window.addEventListener('resize', setWidth);

    function setWidth() {
        let imgWidth = parseFloat(width) + 10;
        let itemCount = Math.floor(window.innerWidth / imgWidth);
        var newWidth = itemCount * imgWidth;
        document.documentElement.style.setProperty('--calculated-width', `${newWidth}px`);
    }
</script>

<!-- TODO: categories -->
<div id="lib-manga-section">
<!-- TODO: loading icon -->
{#each library as l, i}
    <a class="lib-manga" href="/manga/{l.plugin}/{l.id}">
        <div class="lib-manga-wrap">
            <img src={l.img} alt={l.title} on:error={console.log("img failed")}/>
        </div>
        <p>{l.title}</p>
    </a>
{/each}
</div>

<style>
    :root {
        --calculated-width: 100px;
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
        color: var(--text-color);
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
        height: 110%;
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
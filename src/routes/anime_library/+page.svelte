<script>
    import store from "$lib/store.js";

    let library = [];
    let width = document.documentElement.style.getPropertyValue('--lib-anime-width');

    store.subscribe(json => {
        library = json["anime_library"];
        if (json["settings"].hasOwnProperty("library_widths")) {
            if (json["settings"]["library_widths"].hasOwnProperty("anime")) {
                width = json["settings"]["library_widths"].anime;
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
<div id="lib-anime-section">
<!-- TODO: loading icon -->
{#each library as l, i}
    <a class="lib-anime" href="/anime/{l.plugin}/{l.id}">
        <div class="lib-anime-wrap">
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
    #lib-anime-section {
        display: flex;
        flex-wrap: wrap;
        justify-content: flex-start;
        overflow-y: auto;
        margin: auto;
        width: var(--calculated-width);
    }
    .lib-anime {
        text-align: center;
        width: fit-content;
        text-decoration: none;
        color: var(--text-color);
        margin: 5px;
    }
    .lib-anime-wrap {
        display: flex;
        justify-content: center;
        align-items: center;
        width: var(--lib-anime-width);
        height: var(--lib-anime-height);
        border-radius: 5px;
        overflow: hidden;
    }
    .lib-anime-wrap img {
        width: auto;
        height: 110%;
    }
    .lib-anime p {
        margin: 0;
        padding: 0;
        width: var(--lib-anime-width);
        height: 1.5em;
        font-size: x-small;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        line-clamp: 2;
    }
</style>
<script>
    import store from "$lib/store.js";

    let library = [];
    let media_screen = "manga";
    let width = document.documentElement.style.getPropertyValue('--lib-manga-width');

    store.subscribe(json => {
        media_screen = json.media_screen;
        switch (media_screen) {
            case "manga":
                library = json["manga_library"];
                width = json["settings"]["library_widths"].manga;
                setWidth();
                break;
            case "anime":
                library = json["anime_library"];
                width = json["settings"]["library_widths"].anime;
                setWidth();
                break;
            case "ln":
                library = json["ln_library"];
                width = json["settings"]["library_widths"].ln;
                setWidth();
                break;
        }
        document.documentElement.style.setProperty('--lib-width', `${width}px`);
        document.documentElement.style.setProperty('--lib-height', `${width*1.5}px`);
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
<div id="lib-section">
<!-- TODO: loading icon -->
{#each library as l, i}
    <a class="lib" href="/{media_screen}/{l.plugin}/{l.id}">
        <div class="lib-wrap">
            <img src={l.img} alt={l.title} on:error={console.log("img failed")}/>
        </div>
        <p>{l.title}</p>
    </a>
{/each}
</div>

<style>
    :root {
        --calculated-width: 100px;
        --lib-width: 100px;
        --lib-height: calc(var(--lib-width) *1.5);
    }
    #lib-section {
        display: flex;
        flex-wrap: wrap;
        justify-content: flex-start;
        overflow-y: auto;
        margin: auto;
        width: var(--calculated-width);
    }
    .lib {
        text-align: center;
        width: fit-content;
        text-decoration: none;
        color: var(--text-color);
        margin: 5px;
    }
    .lib p {
        margin: 0;
        padding: 0;
        width: var(--lib-width);
        height: 1.5em;
        font-size: x-small;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        line-clamp: 2;
    }
    .lib-wrap {
        display: flex;
        justify-content: center;
        align-items: center;
        width: var(--lib-width);
        height: var(--lib-height);
        border-radius: 5px;
        overflow: hidden;
    }
    .lib-wrap img {
        width: auto;
        height: 110%;
    }
</style>
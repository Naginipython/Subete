<script>
    import store from "$lib/store.js";

    let library = [];
    let width = document.documentElement.style.getPropertyValue('--lib-ln-width');

    store.subscribe(json => {
        library = json["ln_library"];
        if (json["settings"].hasOwnProperty("library_widths")) {
            if (json["settings"]["library_widths"].hasOwnProperty("ln")) {
                width = json["settings"]["library_widths"].ln;
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
<div id="lib-ln-section">
<!-- TODO: loading icon -->
{#each library as l, i}
    <a class="lib-ln" href="/ln/{l.id}">
        <div class="lib-ln-wrap">
            <img src={l.img} alt={l.title}/>
        </div>
        <p>{l.title}</p>
    </a>
{/each}
</div>

<style>
    #lib-ln-section {
        display: flex;
        flex-wrap: wrap;
        justify-content: flex-start;
        overflow-y: auto;
        margin: auto;
        width: var(--calculated-width);
    }
    .lib-ln {
        text-align: center;
        width: fit-content;
        text-decoration: none;
        color: var(--text-color);
        margin: 5px;
    }
    .lib-ln-wrap {
        display: flex;
        justify-content: center;
        align-items: center;
        width: var(--lib-ln-width);
        height: var(--lib-ln-height);
        border-radius: 5px;
        overflow: hidden;
    }
    .lib-ln-wrap img {
        width: auto;
        height: 110%;
    }
    .lib-ln p {
        margin: 0;
        padding: 0;
        width: var(--lib-ln-width);
        height: 1.5em;
        font-size: x-small;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        line-clamp: 2;
    }
</style>
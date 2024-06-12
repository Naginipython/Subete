<script>
    export let data = [];
    export let media_screen = "manga";

    // Drag Scrolling
    let scrollContainer;
    let isDragging = false;
    let startX;
    let scrollLeft;
    const handleMouseDown = (event) => {
        isDragging = true;
        startX = event.pageX - scrollContainer.offsetLeft;
        scrollLeft = scrollContainer.scrollLeft;
    };
    const handleMouseLeave = () => {
        isDragging = false;
    };
    const handleMouseUp = (event) => {
        event.preventDefault();
        isDragging = false;
    };
    const handleMouseMove = (event) => {
        if (!isDragging) return;
        event.preventDefault();
        const x = event.pageX - scrollContainer.offsetLeft;
        const walk = (x - startX) * 1; // Multiply by a factor to increase/decrease scroll speed
        scrollContainer.scrollLeft = scrollLeft - walk;
    };
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<div class="manga-section"
    bind:this={scrollContainer}
    on:mousedown={handleMouseDown}
    on:mouseleave={handleMouseLeave}
    on:mouseup={handleMouseUp}
    on:mousemove={handleMouseMove}
    on:dragstart|preventDefault
    role="group">
    {#each data as d, i} <!-- can change d to {varname varname}, example {img, title}. i exists to find index -->

        <a class="manga" href="/{media_screen}/{d.id}">
            <div class="manga-wrap">
                <img src={d.img} alt={d.title}/>
            </div>
            <p>{d.title}</p>
        </a>

    {/each}
</div>

<style>
    :root {
        --manga-height: 200px; /* pref digit of 100 */
    }
    .manga-section {
        overflow-x: scroll;
        white-space: nowrap;
        cursor: grab;
        display: flex;
    }
    /* May lag on firefox */
    .manga-section::-webkit-scrollbar {
        width: 10px;
        background-color: transparent;
    }
    .manga-section::-webkit-scrollbar-thumb {
        background: #BDBDBD;
        border-radius: 50px;
    }
    .manga-section::-webkit-scrollbar-thumb:hover {
        background: #6E6E6E;
    }

    .manga {
        display: inline-block;
		scroll-snap-align: start;
        text-align: center;
        width: fit-content;
        text-decoration: none;
        color: var(--text-color);
        margin: 5px;
    }
    .manga-wrap {
        display: flex;
        justify-content: center;
        align-items: center;
        width: calc(0.5*var(--manga-height) + 50px);
        height: var(--manga-height);
        border-radius: 5px;
        overflow: hidden;
    }
    .manga-wrap img {
        width: auto;
        height: 105%;
    }
    .manga p {
        margin: 0;
        padding: 0;
        width: calc(0.5*var(--manga-height) + 50px);
        height: 2em;
        font-size: x-small;
        /* white-space: wrap; */
        text-overflow: ellipsis;
        overflow: hidden;
        line-clamp: 2;
    }
</style>
<style>
    /* manga (TO CHANGE) */
    .manga-section {
        overflow: hidden;
        white-space: nowrap;
        cursor: grab;
    }
    .manga {
        padding: 0 5px;
        text-decoration: none;
        color: white;
        display: inline-block;
		height: 100vh;
		scroll-snap-align: start;
		position: relative;
        width: 200px;
    }
    .manga img {
        width: 200px;
    }
    .manga p {
        margin: 0;
        padding: 0;
        width: 200px;
        height: 2em;
        font-size: x-small;
        /* white-space: wrap; */
        text-overflow: ellipsis;
        overflow: hidden;
        line-clamp: 2;
    }
</style>

<script>
    export let data = [];

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
    const handleMouseUp = () => {
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

<!-- display manga (only mangadex rn) -->
<p>MangaDex <br><span style="font-size: small;">English</span></p>
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

        <a class="manga" href="/manga/{d.id}">
            <img src={d.img} alt={d.title}/>
            <p>{d.title}</p>
        </a>

    {/each}
</div>
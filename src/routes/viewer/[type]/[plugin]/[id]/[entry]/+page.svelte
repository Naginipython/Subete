<script>
    import Anime from "./anime.svelte";
    import Manga from "./manga.svelte";
    import Ln from "./ln.svelte";
    export let data;

    let viewer = data.type;

</script>

{#if data.type == "anime"}
    <Anime {data} />
{:else if data.type == "manga"}
    <Manga {data} />
{:else}
    <Ln {data} />
{/if}
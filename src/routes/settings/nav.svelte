<script>
    import { goto } from "$app/navigation";
</script>

<div class="theme-box">
    <button on:click={() => {goto("/settings/nav_ordering")}}>Nav Bar Ordering (MEGA WIP)</button>
</div>

<style>
    .theme-box {
        width: 100vw;
        padding: 10px 0;
        margin: 2px 0;
        overflow: auto;
        display: inline-flex;
        background-color: var(--secondary-color);
        align-items: center;
    }
</style>
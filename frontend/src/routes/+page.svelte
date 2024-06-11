<script>
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    // On mount, prep store and send to set page
    onMount(async () => {
        goto('/manga_library');
    });
</script>
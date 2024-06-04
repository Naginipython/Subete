<script>
    import { onMount } from "svelte";

    // On mount, prep store and send to set page
    onMount(() => {
        window.location.href = '/library';
        // window.location.href = '/browse';
        // window.location.href = '/manga/0/reader/6691898e-fd28-4a82-9f47-86eb77fcefd6';
        // window.location.href = '/manga/a1c7c817-4e59-43b7-9365-09675a149a6f'
    });
</script>
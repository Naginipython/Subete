<script>
    import Theme from "./theme.svelte";
    import Library from "./library.svelte";
</script>

<h3 class="set-header">Settings</h3>

<hr>
<h4 class="set-header">Appearance</h4>
<Theme />

<hr>

<h4 class="set-header">Library</h4>
<Library />

<style>
    hr {
        border-color: grey;
    }
    .set-header {
        margin-left: 10px;
    }
</style>
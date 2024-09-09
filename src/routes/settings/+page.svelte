<script>
    import { invoke } from "@tauri-apps/api/core";
    import store from "$lib/store.js";
    import Theme from "./theme.svelte";
    import Library from "./library.svelte";
    import Nav from "./nav.svelte";
    async function delete_manga_lib() {
        invoke('delete_manga_lib');
        store.update(json => {json.manga_library = []; return json;});
    }
    async function delete_ln_lib() {
        invoke('delete_ln_lib');
        store.update(json => {json.ln_library = []; return json;});
    }
    async function delete_set() {
        invoke('delete_settings');
        store.update(json => {json.settings = []; return json;});
    }
</script>

<h3 class="set-header">Settings</h3>

<hr>
<h4 class="set-header">Appearance</h4>
<Theme />

<hr>

<h4 class="set-header">Library</h4>
<Library />

<h4 class="set-header">Nav Bar</h4>
<Nav />

<hr>

<!-- TODO: double check delete -->
<h4 class="set-header">Data</h4>
<div class="settings-box">
    <button class="delete-btn" on:click={async () => delete_manga_lib()}>Delete Manga Library</button>
</div>
<div class="settings-box">
    <button class="delete-btn" on:click={async () => delete_ln_lib()}>Delete LN Library</button>
</div>
<div class="settings-box">
    <button class="delete-btn" on:click={async () => await invoke('delete_manga_plugins')}>Delete Manga Plugins</button>
</div>
<div class="settings-box">
    <button class="delete-btn" on:click={async () => await invoke('delete_ln_plugins')}>Delete LN Plugins</button>
</div>
<div class="settings-box">
    <button class="delete-btn" on:click={async () => delete_set()}>Delete Settings</button>
</div>

<style>
    hr {
        border-color: grey;
    }
    .set-header {
        margin-left: 10px;
    }
    .settings-box {
        width: 100vw;
        overflow: auto;
        display: inline-flex;
        background-color: var(--secondary-color);
    }
    .settings-box:hover {
        background-color: var(--selection-color)
    }
    .delete-btn {
        cursor: pointer;
        display: inline-flex;
        justify-content: left;
        align-items: center;
        border: 0;
        background-color: transparent;
        color: inherit;
        width: 100%;
        padding: 10px 0;
        font-size: large;
    }
</style>
<script>
    import store from "$lib/store.js";
    import { invoke } from "@tauri-apps/api/core";

    let library_widths = {
        manga: '',
        ln: '',
    };
    $: manga_item_custom = manga_width == "Custom";
    $: ln_item_custom = ln_width == "Custom";
    let manga_width = "2x"; // 100 + 50*width
    let ln_width = "2x"; // 100 + 50*width
    let init = true;

    store.subscribe(async json => {
        if (!json["settings"].hasOwnProperty("library_widths")) {
            library_widths = { manga: '100', ln: '100' };
        } else {
            if (!json['settings']['library_widths'].hasOwnProperty("manga")) {
                library_widths.manga = "100";
            } else {
                library_widths.manga = json["settings"]["library_widths"].manga;
            }
            if (!json['settings']['library_widths'].hasOwnProperty("ln")) {
                library_widths.ln = "100";
            } else {
                library_widths.ln = json["settings"]["library_widths"].ln;
            }
        }
        if (init) {
            init = false;
            switch(library_widths.manga) {
                case "50": manga_width = "1x"; break;
                case "100": manga_width = "2x"; break;
                case "150": manga_width = "3x"; break;
                case "200": manga_width = "4x"; break;
                case "250": manga_width = "5x"; break;
                default: manga_width = "Custom";
            }
            switch(library_widths.ln) {
                case "50": ln_width = "1x"; break;
                case "100": ln_width = "2x"; break;
                case "150": ln_width = "3x"; break;
                case "200": ln_width = "4x"; break;
                case "250": ln_width = "5x"; break;
                default: ln_width = "Custom";
            }
        }
    });

    async function change_manga_width() {
        if (manga_width != "Custom") {
            let scale = parseInt(manga_width[0]);
            library_widths.manga = (50*scale).toString();
        }
        document.documentElement.style.setProperty('--lib-manga-width', `${library_widths.manga}px`);
        store.update(json => {
            json["settings"].library_widths = library_widths;
            return json;
        });
        await invoke('update_settings', { newSettings: {"library_widths": library_widths}})
    }
    async function change_ln_width() {
        if (ln_width != "Custom") {
            let scale = parseInt(ln_width[0]);
            library_widths.ln = (50*scale).toString();
        }
        document.documentElement.style.setProperty('--lib-ln-width', `${library_widths.ln}px`);
        store.update(json => {
            json["settings"].library_widths = library_widths;
            return json;
        });
        await invoke('update_settings', { newSettings: {"library_widths": library_widths}})
    }
</script>

<div class="library-box">
    <label for="presets">Manga Item Size: </label>
    <select id="presets" bind:value={manga_width} on:change={async () => change_manga_width()}>
        <option>1x</option>
        <option>2x</option>
        <option>3x</option>
        <option>4x</option>
        <option>5x</option>
        <option>Custom</option>
    </select>
    {#if manga_width == "Custom"}
        <form on:submit={change_manga_width}>
            <input id="input-primary-color" bind:value={library_widths.manga} /><span>px</span>
            <input type="button" value="Apply"/>
        </form>
    {/if}
</div>
<div class="library-box">
    <label for="presets">Light Novel Item Size: </label>
    <select id="presets" bind:value={ln_width} on:change={async () => change_ln_width()}>
        <option>1x</option>
        <option>2x</option>
        <option>3x</option>
        <option>4x</option>
        <option>5x</option>
        <option>Custom</option>
    </select>
    {#if ln_width == "Custom"}
        <form on:submit={change_ln_width}>
            <input id="input-primary-color" bind:value={library_widths.ln} /><span>px</span>
            <input type="button" value="Apply"/>
        </form>
    {/if}
</div>

<style>
    .library-box {
        width: 100vw;
        padding: 10px 0;
        margin: 2px 0;
        overflow: auto;
        display: inline-flex;
        background-color: var(--secondary-color);
        align-items: center;
    }
    .library-box label {
        height: fit-content;
        padding: 0 5px;
    }
    .library-box form {
        margin-left: 25px;
    }
</style>
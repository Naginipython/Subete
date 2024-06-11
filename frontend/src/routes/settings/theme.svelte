<script>
    import store from "$lib/store.js";
    import { invoke } from "@tauri-apps/api/core";

    let color_text = {
        primary: '',
        secondary: '',
        selection: '',
        text: '',
    };
    let primary_color_select = "Dark";
    let secondary_color_select = "Shitty Red";
    let selection_color_select = "Shitty Red";
    let text_color_select = "White";
    let init = true;

    store.subscribe(async json => {
        if (!json["settings"].hasOwnProperty("app_colors")) {
            color_text = {primary: "1a1a1a", secondary: "330000", selection: "800000", text: "ffffff"};
        } else {
            if (!json['settings']['app_colors'].hasOwnProperty("primary")) {
                color_text.primary = "1a1a1a";
            } else {
                color_text.primary = json["settings"]["app_colors"].primary;
            }
            if (!json['settings']['app_colors'].hasOwnProperty("secondary")) {
                color_text.secondary = "330000";
            } else {
                color_text.secondary = json["settings"]["app_colors"].secondary;
            }
            if (!json['settings']['app_colors'].hasOwnProperty("selection")) {
                color_text.selection = "800000";
            } else {
                color_text.selection = json["settings"]["app_colors"].selection;
            }
            if (!json['settings']['app_colors'].hasOwnProperty("text")) {
                color_text.text = "ffffff";
            } else {
                color_text.text = json["settings"]["app_colors"].text;
            }
        }
        if (init) {
            init = false;
            switch (color_text.primary) {
                case "1a1a1a": primary_color_select = "Dark"; break;
                case "f2f2f2": primary_color_select = "Light"; break;
                default: primary_color_select = "Other";
            }
            switch (color_text.secondary) {
                case "330000": secondary_color_select = "Shitty Red"; break;
                case "331f00": secondary_color_select = "Shitty Orange"; break;
                case "333300": secondary_color_select = "Shitty Yellow"; break;
                case "003300": secondary_color_select = "Shitty Green"; break;
                case "000033": secondary_color_select = "Shitty Blue"; break;
                case "1a0033": secondary_color_select = "Shitty Purple"; break;
                default: secondary_color_select = "Other";
            }
            switch (color_text.selection) {
                case "800000": selection_color_select = "Shitty Red"; break;
                case "801f00": selection_color_select = "Shitty Orange"; break;
                case "808000": selection_color_select = "Shitty Yellow"; break;
                case "008000": selection_color_select = "Shitty Green"; break;
                case "000080": selection_color_select = "Shitty Blue"; break;
                case "1a0080": selection_color_select = "Shitty Purple"; break;
                default: selection_color_select = "Other";
            }
            switch (color_text.text) {
                case "ffffff": text_color_select = "White"; break;
                case "000000": text_color_select = "Black"; break;
                default: text_color_select = "Other";
            }
        }
        return json;
    });

    // PRIMARY COLOR
    async function change_primary_color() {
        switch (primary_color_select) {
            case "Dark":
                color_text.primary = "1a1a1a";
                document.documentElement.style.setProperty('--primary-color', `#1a1a1a`); 
                break;
            case "Light":
                color_text.primary = "f2f2f2";
                document.documentElement.style.setProperty('--primary-color', `#f2f2f2`); 
                break;
            default:
                if (color_text.primary.length == 6 && parseInt(color_text.primary, 16) != null) {
                    document.documentElement.style.setProperty('--primary-color', `#${color_text.primary}`);
                }
        }
        store.update(json => {
            json["settings"].app_colors = color_text;
            console.log(json['settings']);
            return json;
        });
        await invoke('update_settings', { newSettings: {"app_colors":color_text}})
    }

    // SECONDARY COLOR
    async function change_secondary_color() {
        switch (secondary_color_select) {
            case "Shitty Red":
                color_text.secondary = "330000";
                document.documentElement.style.setProperty('--secondary-color', `#330000`); 
                break;
            case "Shitty Orange":
                color_text.secondary = "331f00";
                document.documentElement.style.setProperty('--secondary-color', `#331f00`); 
                break;
            case "Shitty Yellow":
                color_text.secondary = "333300";
                document.documentElement.style.setProperty('--secondary-color', `#333300`); 
                break;
            case "Shitty Green":
                color_text.secondary = "003300";
                document.documentElement.style.setProperty('--secondary-color', `#003300`); 
                break;
            case "Shitty Blue":
                color_text.secondary = "000033";
                document.documentElement.style.setProperty('--secondary-color', `#000033`); 
                break;
            case "Shitty Purple":
                color_text.secondary = "1a0033";
                document.documentElement.style.setProperty('--secondary-color', `#1a0033`); 
                break;
            default:
                if (color_text.secondary.length == 6 && parseInt(color_text.secondary, 16) != null) {
                    document.documentElement.style.setProperty('--secondary-color', `#${color_text.secondary}`);
                }
        }
        store.update(json => {
            json["settings"].app_colors = color_text;
            console.log(json['settings']);
            return json;
        });
        await invoke('update_settings', { newSettings: {"app_colors":color_text}})
    }

    // SELECTION COLOR
    async function change_selection_color() {
        switch (selection_color_select) {
            case "Shitty Red":
                color_text.selection = "800000";
                document.documentElement.style.setProperty('--selection-color', `#800000`); 
                break;
            case "Shitty Orange":
                color_text.selection = "801f00";
                document.documentElement.style.setProperty('--selection-color', `#801f00`); 
                break;
            case "Shitty Yellow":
                color_text.selection = "808000";
                document.documentElement.style.setProperty('--selection-color', `#808000`); 
                break;
            case "Shitty Green":
                color_text.selection = "008000";
                document.documentElement.style.setProperty('--selection-color', `#008000`); 
                break;
            case "Shitty Blue":
                color_text.selection = "000080";
                document.documentElement.style.setProperty('--selection-color', `#000080`); 
                break;
            case "Shitty Purple":
                color_text.selection = "1a0080";
                document.documentElement.style.setProperty('--selection-color', `#1a0080`); 
                break;
            default:
                if (color_text.selection.length == 6 && parseInt(color_text.selection, 16) != null) {
                    document.documentElement.style.setProperty('--selection-color', `#${color_text.selection}`);
                }
        }
        store.update(json => {
            json["settings"].app_colors = color_text;
            console.log(json['settings']);
            return json;
        });
        await invoke('update_settings', { newSettings: {"app_colors":color_text}})
    }

    // TEXT COLOR
    async function change_text_color() {
        switch (text_color_select) {
            case "Black": 
                color_text.text = "000000";
                document.documentElement.style.setProperty('--text-color', `#000000`); 
                break;
            case "White": 
                color_text.text = "ffffff";
                document.documentElement.style.setProperty('--text-color', `#ffffff`); 
                break;
            default:
                if (color_text.text.length == 6 && parseInt(color_text.text, 16) != null) {
                    document.documentElement.style.setProperty('--text-color', `#${color_text.text}`);
                }
        }
        store.update(json => {
            json["settings"].app_colors = color_text;
            return json;
        });
        await invoke('update_settings', { newSettings: {"app_colors":color_text}})
    }
</script>

<div class="theme-box">
    <label for="primary-color">Choose Primary Color: </label>
    <select id="primary-color" bind:value={primary_color_select} on:change={async () => change_primary_color()}>
        <option>Dark</option>
        <option>Light</option>
        <option>Other</option>
    </select>
    {#if primary_color_select == "Other"}
        <form on:submit={change_primary_color} class="theme-option-input">
            <span>#</span><input id="input-primary-color" bind:value={color_text.primary} on:change={async () => change_primary_color()} />
            <input type="button" value="Apply"/>
        </form>
    {/if}
</div>

<div class="theme-box">
    <label for="secondary-color">Choose Secondary Color: </label>
    <select id="secondary-color" bind:value={secondary_color_select} on:change={async () => change_secondary_color()}>
        <option>Shitty Red</option>
        <option>Shitty Orange</option>
        <option>Shitty Yellow</option>
        <option>Shitty Green</option>
        <option>Shitty Blue</option>
        <option>Shitty Purple</option>
        <option>Other</option>
    </select>
    {#if secondary_color_select == "Other"}
        <form on:submit={change_secondary_color} class="theme-option-input">
            <span>#</span><input id="input-secondary-color" bind:value={color_text.secondary} on:change={async () => change_secondary_color()} />
            <input type="button" value="Apply"/>
        </form>
    {/if}
</div>

<div class="theme-box">
    <label for="selection-color">Choose selection Color: </label>
    <select id="selection-color" bind:value={selection_color_select} on:change={async () => change_selection_color()}>
        <option>Shitty Red</option>
        <option>Shitty Orange</option>
        <option>Shitty Yellow</option>
        <option>Shitty Green</option>
        <option>Shitty Blue</option>
        <option>Shitty Purple</option>
        <option>Other</option>
    </select>
    {#if selection_color_select == "Other"}
        <form on:submit={change_selection_color} class="theme-option-input">
            <span>#</span><input id="input-selection-color" bind:value={color_text.selection} on:change={async () => change_selection_color()} />
            <input type="button" value="Apply"/>
        </form>
    {/if}
</div>

<div class="theme-box">
    <label for="text-color">Choose Text Color: </label>
    <select id="text-color" bind:value={text_color_select} on:change={async () => change_text_color()}>
        <option>Black</option>
        <option>White</option>
        <option>Other</option>
    </select>
    {#if text_color_select == "Other"}
        <form on:submit={async () => change_text_color()} class="theme-option-input">
            <span>#</span><input id="input-text-color" bind:value={color_text.text} on:change={async () => change_text_color()} />
            <input type="button" value="Apply"/>
        </form>
    {/if}
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
    .theme-box label {
        height: fit-content;
        padding: 0 5px;
    }
    .theme-option-input {
        margin-left: 25px;
    }
</style>
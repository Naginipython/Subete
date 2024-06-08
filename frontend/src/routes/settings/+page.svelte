<script>
    import store from "$lib/store.js";
    import { invoke } from '@tauri-apps/api/tauri';

    let color_text = {
        secondary: '',
        text: '',
    };
    let secondary_color_select = "Shitty Red";
    let text_color_select = "White";

    store.subscribe(async json => {
        console.log(json['settings']);
        if (!json["settings"].hasOwnProperty("app_colors")) {
            color_text = {secondary: "330000", text: "ffffff"};
        } else {
            if (!json['settings']['app_colors'].hasOwnProperty("secondary")) {
                color_text.secondary = "330000";
            } else {
                color_text.secondary = json["settings"]["app_colors"].secondary; // "330000";
            }
            if (!json['settings']['app_colors'].hasOwnProperty("text")) {
                color_text.text = "ffffff";
            } else {
                color_text.text = json["settings"]["app_colors"].text; // "ffffff";
            }
        }
        switch (color_text.secondary) {
            case "330000": secondary_color_select = "Shitty Red"; break;
            case "331f00": secondary_color_select = "Shitty Orange"; break;
            case "333300": secondary_color_select = "Shitty Yellow"; break;
            case "003300": secondary_color_select = "Shitty Green"; break;
            case "000033": secondary_color_select = "Shitty Blue"; break;
            case "1a0033": secondary_color_select = "Shitty Purple"; break;
        }
        switch (color_text.secondary) {
            case "ffffff": secondary_color_select = "White"; break;
            case "000000": secondary_color_select = "Black"; break;
        }
        return json;
    });

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
        console.log(color_text);
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

<div>
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
        <form on:submit={change_secondary_color}>
            <span>#</span><input id="input-secondary-color" bind:value={color_text.secondary} on:change={async () => change_secondary_color()} />
            <input type="button" value="Apply"/>
        </form>
    {/if}
</div>
<div>
    <label for="text-color">Choose Text Color: </label>
    <select id="text-color" bind:value={text_color_select} on:change={async () => change_text_color()}>
        <option>Black</option>
        <option>White</option>
        <option>Other</option>
    </select>
    {#if text_color_select == "Other"}
        <form on:submit={async () => change_text_color()}>
            <span>#</span><input id="input-text-color" bind:value={color_text.text} on:change={async () => change_text_color()} />
            <input type="button" value="Apply"/>
        </form>
    {/if}
    <!-- var(--text-color); -->
</div>
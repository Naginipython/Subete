<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Fa from 'svelte-fa'
  import { faXmark } from '@fortawesome/free-solid-svg-icons'

  let manga_plugins = [];
  let ln_plugins = [];
  let anime_plugins = [];
  onMount(async () => {
    manga_plugins = await invoke("get_manga_plugin_names");
    anime_plugins = await invoke("get_anime_plugin_names");
    ln_plugins = await invoke("get_ln_plugin_names");
  });

  async function remove_plugin(media_type, plugin) {
    switch (media_type) {
      case 'manga':
        await invoke('delete_manga_plugin', { plugin })
        manga_plugins = await invoke("get_manga_plugin_names");
        break;
      case 'ln':
        await invoke('delete_ln_plugin', { plugin })
        ln_plugins = await invoke("get_ln_plugin_names");
        break;
    }
  }

  let files;
  let json = {error: "", success: ""};
  let media_type = "manga";
  async function validateFile(file) {
    const plugin = file.name.split('.').pop().toLowerCase();
    if (plugin !== 'json') {
      alert('Only .json files are allowed!');
      return false;
    }
    const text = await file.text();
    try {
      const test_json = JSON.parse(text);
      if (
        test_json.hasOwnProperty("id") &&
        test_json.hasOwnProperty("media_type") &&
        test_json.hasOwnProperty("search_url") &&
        test_json.hasOwnProperty("search") &&
        test_json.hasOwnProperty("search_extra") &&
        (test_json.hasOwnProperty("chapters_url") || test_json.hasOwnProperty("episodes_url")) &&
        (test_json.hasOwnProperty("get_chapters") || test_json.hasOwnProperty("get_episodes")) &&
        (test_json.hasOwnProperty("chapters_extra") || test_json.hasOwnProperty("episodes_extra")) &&
        (test_json.hasOwnProperty("pages_url") || test_json.hasOwnProperty("videos_url")) &&
        (test_json.hasOwnProperty("get_pages") || test_json.hasOwnProperty("get_videos")) &&
        (test_json.hasOwnProperty("pages_extra") || test_json.hasOwnProperty("videos_extra"))
      ) {
        json = test_json;
        media_type = test_json.media_type;
        json.error = "";
        json.success = "File uploaded is valid";
      } else {
        json.error = "ERROR: Does not contain needed field(s)"
        console.log(test_json);
      }
    } catch (error) {
      console.error('Invalid JSON file:', error);
    }
  }
  async function submit() {
    if (!files || !validateFile(files[0])) return;
    // Replace with actual server-side logic for saving the file
    console.log('File uploaded:', files[0]);
    json.error = "";
    json.success = "";
    // console.log(json);
    switch (media_type) {
      case "manga":
        json.success = `Manga plugin uploaded: ${json.id}`;
        await invoke('add_manga_plugin', { newPlugin: json });
        manga_plugins = await invoke("get_manga_plugin_names");
        break;
      case "ln":
        json.success = `Ln plugin uploaded: ${json.id}`;
        await invoke('add_ln_plugin', { newPlugin: json });
        ln_plugins = await invoke("get_ln_plugin_names");
        break;
      case "anime":
        json.success = `Anime plugin uploaded: ${json.id}`;
        await invoke('add_anime_plugin', { newPlugin: json });
        anime_plugins = await invoke("get_anime_plugin_names");
        break;
      default:
        json.error = "ERROR: media_type is invalid";
    }
    
  }
  console.log(json);
</script>

<h3 id="plugin-uploader">Plugin Uploader</h3>
<form on:submit={submit} id="plugin-form">
    <label for="source">Please upload a JSON file:</label>
    <input type="file" id="source" bind:files on:change={async () => validateFile(files[0])} />
    <button type="submit" on:click={async () => submit()}>Upload</button>
    {#if json.error != ""}
      <p style="color: red">{json.error}</p>
    {:else if json.success != ""}
      <p style="color: green">{json.success}</p>
    {/if}
</form>

<div id="plugin-show">
  <div class="plugin-show-column">
    <h4>Manga</h4>
    {#each manga_plugins as plugin}
      <div class="plugin-show-item">
        <p>{plugin}</p>
        <button on:click={async () => await remove_plugin('manga', plugin)}><Fa icon={faXmark} /></button>
      </div>
    {/each}
  </div>
  <div class="plugin-show-column">
    <h4>Anime</h4>
    {#each anime_plugins as plugin}
      <div class="plugin-show-item">
        <p>{plugin}</p>
        <button on:click={async () => await remove_plugin('anime', plugin)}><Fa icon={faXmark} /></button>
      </div>
    {/each}
  </div>
  <div class="plugin-show-column">
    <h4>Light Novel</h4>
    {#each ln_plugins as plugin}
      <div class="plugin-show-item">
        <p>{plugin}</p>
        <button on:click={async () => await remove_plugin('ln', plugin)}><Fa icon={faXmark} /></button>
      </div>
    {/each}
  </div>
</div>

<style>
  #plugin-uploader {
    width: fit-content;
    margin: auto;
    padding: 10px;
  }
  #plugin-form {
    display: flex;
    width: fit-content;
    height: fit-content;
    margin: auto;
    align-items: center;
    flex-direction: column;
  }
  #plugin-show {
    display: flex;
    width: fit-content;
    margin: auto;
  }
  .plugin-show-column {
    padding: 10px;
    width: 30vw;
  }
  .plugin-show-column h4 {
    text-align: center;
  }
  .plugin-show-item {
    display: flex;
    justify-content: space-between;
  }
  .plugin-show-item p {
    margin: 0;
    padding: 0;
  }
  .plugin-show-item button {
    display: inline-flex;
    align-items: center;
    background-color: transparent;
    border: 0;
    color: var(--text-color);
    font-size: large;
    height: fit-content;
    margin: 0 2px;
  }
  .plugin-show-item button:hover {
    background-color: var(--selection-color);
    border-radius: 20px;
  }
</style>
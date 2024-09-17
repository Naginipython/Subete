<script>
  import { invoke } from "@tauri-apps/api/core";
  import { platform } from '@tauri-apps/plugin-os';
  import { onMount } from 'svelte';
  import { find_item } from "$lib/common.js";
  import store from "$lib/store.js";
  import Hls from 'hls.js';
  
  // svelte-ignore unused-export-let
  export let data;

  let anime = {};
  let episode = {};
  let is_loading = false;
  let currPlatform = "windows";

  let player;
  let hls;
  let time = 0;
  let duration;
  let paused = true;

  // URL to the .m3u8 file
  let streamUrl = 'https://www048.anzeat.pro/streamhls/c2039a66c65a3592163a68424f5cf8ea/ep.6.1709540076.m3u8';

  onMount(async () => {
    is_loading = true;
    currPlatform = await platform();
    // TODO: add query params for this, but if there are none do find manga
    anime = find_item("anime", data.plugin, data.id);
    episode = anime['episodes'][data.anime_index];
    streamUrl = await invoke('get_anime_video', { source: anime.plugin, id: episode.id });

    if (currPlatform == "linux") {
      await invoke("open_in_vlc", { url: streamUrl });
    } else {
      // Sets up link with video
      if (Hls.isSupported()) {
        hls = new Hls();
        hls.loadSource(streamUrl);
        hls.attachMedia(player);
        hls.on(Hls.Events.MANIFEST_PARSED, () => {
          player.play();
        });
      } else if (player.canPlayType('application/vnd.apple.mpegurl')) {
        // Native support (e.g., in Safari)
        player.src = streamUrl;
        player.play();
      }
    }
    is_loading = false;
  });

  function keyInput(event) {
    if (event.key == ' ') {
      event.preventDefault();
      console.log("paused");
      if (paused) {
        player.play();
      } else {
        player.pause();
      }
    }
  }

  // window.addEventListener('resize', adjustImage);
  function adjustImage() {
    const vidAspectRatio = player.videoWidth / player.videoHeight;
    const viewportAspectRatio = window.innerWidth / window.innerHeight;
    
    if (vidAspectRatio > viewportAspectRatio) {
      player.style.height = "auto";
      player.style.width = "100vw";
    } else {
      player.style.height = "100vh";
      player.style.width = "auto";
    }
  }
</script>

<svelte:window on:keydown={keyInput} on:resize={adjustImage}/>

<div id="player-box">
  <!-- svelte-ignore a11y-media-has-caption -->
   {#if currPlatform == "linux"}
    <p>Linux is not supported, vlc opening</p>
  {:else}
    <video 
      bind:this={player}
      id="player"
      controls
      bind:currentTime={time}
      bind:duration
      bind:paused
    />
  {/if}
</div>

<style>
  #player-box {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
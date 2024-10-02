<script>
  import { invoke } from "@tauri-apps/api/core";
  import { platform } from '@tauri-apps/plugin-os';
  import { 
    faArrowLeft, faBookmark, faEllipsisVertical, 
    faPause, faPlay, faRightFromBracket, faExpand
  } from '@fortawesome/free-solid-svg-icons';
  import { faBookmark as faOutlineBookmark } from '@fortawesome/free-regular-svg-icons';
  import { onMount } from 'svelte';
  import { find_item } from "$lib/common.js";
  import appHist from "$lib/history.js";
  import { goto } from "$app/navigation";
  import { Moon } from 'svelte-loading-spinners';
  import Fa from 'svelte-fa'
  import store from "$lib/store.js";
  import Hls from 'hls.js';
  export let data;
  
  let anime = {};
  let episode = {};
  let currPlatform = "windows";
  let is_loading = false;
  let error = "";
  $: if (error != "") {
    setTimeout(() => {
      error = "";
    }, 3000)
  }
  
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
    episode = anime['episodes'][data.index];
    streamUrl = await invoke('get_anime_video', { source: anime.plugin, id: episode.id });
  
    // Sets up link with video
    if (Hls.isSupported()) {
      hls = new Hls({
        forceKeyFrameOnDiscontinuity: true,
        enableWorker: true,
        fragLoadingTimeOut: 20000,
        startFragPrefetch: true, // Prefetch the next fragment before the current one finishes
        appendErrorMaxRetry: 3,
        maxBufferHole: 2, // Set a maximum allowed gap of 2 seconds in the buffer
        maxSeekHole: 2,   // Maximum gap allowed when seeking
        maxBufferLength: 120, // Increase buffer length in seconds
        maxMaxBufferLength: 300, // Max buffer length cap
        lowLatencyMode: false, // Disable low latency mode
        liveSyncDuration: 10, // Duration to maintain a buffer in seconds for live streams
        liveMaxLatencyDuration: 30, // Maximum latency duration for live streams
        maxBufferHole: 1.5, // Maximum allowed gap in buffer (in seconds)
        // maxBufferSize: 320 * 1000 * 1000
      });
      hls.loadSource(streamUrl);
      hls.attachMedia(player);
      hls.on(Hls.Events.MANIFEST_PARSED, () => {
        // player.currentTime = 100;
        player.currentTime = episode.watch_time? episode.watch_time : 0;
        player.play();
        player.style.display = "block";
      });
      hls.on(Hls.Events.ERROR, function (event, errData) {
        error = errData.details;
        console.log(errData);

        if (errData.fatal) {
          switch (errData.type) {
            case Hls.ErrorTypes.NETWORK_ERROR:
              console.log("Network error, trying to recover...");
              hls.startLoad();
              error = "";
              break;
            case Hls.ErrorTypes.MEDIA_ERROR:
              console.log("Media error, attempting recovery...");
              hls.recoverMediaError();
              error = "";
              break;
            default:
              console.log("Cannot recover, destroying HLS instance.");
              hls.destroy();
              error = "";
              break;
          }
        }
      });
    } else if (player.canPlayType('application/vnd.apple.mpegurl')) {
      // Native support (e.g., in Safari)
      player.src = streamUrl;
      player.play();
    }
    is_loading = false;
  });
  
  function keyInput(event) {
    console.log(event.key);
    event.preventDefault();
    switch (event.key) {
      case ' ':
        pause()
        break;
      case 'ArrowRight':
        player.currentTime += 10;
        break;
      case 'ArrowLeft':
        player.currentTime -= 10;
        break;
      case 'i':
        toggle_menu()
        break;
      case 'f':
        openFullscreen();
        break;
    }
  }

  // Snackbar buttons
  async function vlc() {
    player.pause();
    await invoke("open_in_vlc", { url: streamUrl });
  }

  function pause() {
    if (player.readyState < 2) { // If the media is not ready to play
      console.log('Media is not ready, cannot pause');
      return;
    }
    if (paused) {
      player.play();
    } else {
      player.pause();
    }
  }
  function seek(e) {
    let width = document.getElementById("seeker").clientWidth;
    time = duration * (e.offsetX / width);
  }
  function skip() {
    player.currentTime += 85;
  }
  function openFullscreen() {
    if (player.requestFullscreen) {
      player.requestFullscreen();
    } else if (player.webkitRequestFullscreen) { /* Safari */
      player.webkitRequestFullscreen();
    } else if (player.msRequestFullscreen) { /* IE11 */
      player.msRequestFullscreen();
    }
  }

  // Updates library backend
  async function update_lib() {
    // Updates anime
    if (episode.duration == 0 && duration) {
      episode.duration = duration;
    }
    if (time == duration) {
      episode.completed = true;
      episode.watch_time = 0;
      await invoke('update_anime_lib', { item: anime });
    } else if (duration) {
      episode.watch_time = player.currentTime;
      await invoke('update_anime_lib', { item: anime });
    }

    // Updates history
    let new_hist = {
      id: anime.id,
      title: anime.title,
      img: anime.img,
      plugin: anime.plugin,
      recent_episode_id: episode.id,
      recent_episode_num: episode.number,
      timestamp: Date.now()
    }
    // await invoke('save_manga_history', {item: new_hist});
    store.update(json => {
      json.anime_history = json.anime_history.filter(i => !(i.id == new_hist.id && i.plugin == new_hist.plugin));
      json.anime_history.unshift(new_hist);
      return json;
    });
    goto(appHist.back());
  }

  function toggle_menu() {
    let ele_styles = document.getElementById("ep-menu").style;
    ele_styles.opacity = ele_styles.opacity == "1"? "0" : "1";
  }
</script>
  
<svelte:window on:keydown={keyInput} />

<div class="ep-center" style="display: {is_loading? 'flex' : 'none'}">
    <Moon color="var(--selection-color)" size="50" />
</div>

<div id="ep-menu" style="opacity: 0">
  <!-- Snackbar -->
  <div class="menu-background"></div>
  <div id="ep-snackbar">
    <button class="ep-snack-item" on:click={update_lib}><Fa icon={faArrowLeft} /></button>
    <div id="ep-snack-text">
      <p>{anime.title}</p>
      <p style="font-size: x-small">
        {#if episode.title == ""}
          Episode {episode.number}
        {:else}
          Episode {episode.number}: {episode.title}
        {/if}
      </p>
    </div>
    <div class="ep-snack-right">
      <button class="ep-snack-item"><Fa icon={faOutlineBookmark} /></button>
      <button class="ep-snack-item" on:click={async () => {vlc()}}><Fa icon={faRightFromBracket} /></button>
      <button class="ep-snack-item"><Fa icon={faEllipsisVertical} /></button>
    </div>
  </div>

  <button id="show-arrow" on:click={toggle_menu}></button>

  <!-- Media Buttons -->
  <div class="ep-center">
    <button id="pause" on:click={pause}><Fa icon={paused? faPlay : faPause} /></button>
  </div>

  <div class="ep-bot">
    <!-- Top bot -->
    <div id="video-features">
      <div class="ep-snack-right">
        <div style="display: flex; justify-content: center; align-items:center">
          <button class="ep-snack-item ep-snack-item-85s" on:click={skip}>+85 s</button>
          <button class="ep-snack-item no-hover" on:click={openFullscreen}><Fa icon={faExpand} /></button>
        </div>
      </div>
    </div>
    <!-- Bottom bot -->
    <div id="video-time">
      <span>{parseInt(time/60)>0? parseInt(time/60) : 0}:{(parseInt(time)%60).toString().padStart(2, '0')}</span>
      <!-- svelte-ignore a11y-no-noninteractive-element-interactions a11y-click-events-have-key-events a11y-no-static-element-interactions -->
      <div id="seeker" on:click={seek}>
        <div id="seeker-bar" style="width: {(time / duration)*100 || 0}%">
        </div>
      </div>
      <span>{duration? parseInt(duration/60) : 0}:{duration? (parseInt(duration)%60).toString().padStart(2, '0') : '00'}</span>
    </div>
  </div>
</div>

<div id="player-box">
    <!-- svelte-ignore a11y-media-has-caption -->
    <video 
      bind:this={player}
      id="player"
      bind:currentTime={time}
      bind:duration
      bind:paused
      style="width: 100%; display: none;"
    />
  {#if error != ""}
    <p class="error">Error: {error}</p>
  {/if}
</div>
  
<style>
  #player-box {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    z-index: 0;
    pointer-events: none;
  }
  .error {
    color: red; 
    width: inherit; 
    text-align: center; 
    padding: 0; 
    margin: 0;
    position: absolute;
    bottom: 20px;
    background-color: gainsboro;
    border-radius: 50px;
    width: fit-content;
    padding: 10px;
    opacity: 75%;
    pointer-events: none;
  }
  #ep-menu {
    position: absolute;
    width: 100vw;
    height: 100vh;
  }
  #ep-snackbar {
    /* opacity: 0.5; */
    height: var(--snackbar-height);
    /* background-color: var(--secondary-color); */
    position: absolute;
    width: 100vw;
  }
  .menu-background {
    width: 100vw;
    position: absolute;
    opacity: 0.5;
    height: var(--snackbar-height);
    background-color: var(--secondary-color);
  }
  .ep-snack-item {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background-color: transparent;
    border: 0;
    color: var(--text-color);
    font-size: medium;
    height: inherit;
    width: 50px;
  }
  button.ep-snack-item:hover {
    background-color: var(--selection-color);
  }
  button.no-hover:hover {
    background-color: transparent;
  }
  #ep-snack-text {
    position: relative;
    top: -9px;
    display: inline-flex; 
    height: inherit; 
    flex-direction: column;
    width: calc(100vw - 5*50px);
  }
  #ep-snack-text p {
    padding: 0;
    margin: 0;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }
  .ep-snack-right {
    float: right; 
    height: inherit
  }
  #show-arrow {
    position: relative;
    width: 100vw;
    opacity: 0%;
    margin: auto;
    height: calc(100vh - calc(var(--snackbar-height)));
    top: calc(var(--snackbar-height));
  }
  .ep-center {
    position: absolute;
    top: 0;
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    pointer-events: none;
  }
  #pause {
    width: 50px;
    height: 50px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    /* background-color: transparent; */
    border: 0;
    color: black;
    font-size: xx-large;
    width: 50px;
    border-radius: 25px;
    opacity: 50%;
    pointer-events: all;
  }
  .ep-bot {
    width: 100vw;
    height: fit-content;
    margin-bottom: 15px;
    position: absolute;
    bottom: 0;
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
  .ep-snack-item-85s {
    background-color: var(--selection-color);
    width: 70px;
    border-radius: 50px;
    padding: 10px;
  }
  #video-time {
    width: calc(100% - 20px);
    margin-left: 10px;
    display: flex;
    justify-content: center;
    align-items: center;
    /* gap: calc(100vw - 120px); */
    color: var(--text-color)
  }
  #seeker {
    width: calc(100vw - 50px);
    margin: 0 10px;
    cursor: pointer;
    background-color: white;
    border-radius: 20px;
    height: 5px;
    position: relative;
  }
  #seeker-bar {
    background-color: var(--selection-color); 
    color: white;
    text-align: right; 
    font-size: 20px; 
    border-radius: 15px; 
    height: inherit;
    position: relative;
    padding-right: 5px;
  }
</style>
<script>
  import { invoke } from "@tauri-apps/api/core";
  import { platform } from '@tauri-apps/plugin-os';
  import { 
    faArrowLeft, faBookmark, faEllipsisVertical, 
    faPause, faPlay, faRightFromBracket
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
  
    // if (currPlatform == "linux") {
      // await invoke("open_in_vlc", { url: streamUrl });
    // } else {
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
          player.play();
        });
        hls.on(Hls.Events.ERROR, function (event, errData) {
          error = errData.details;
          console.log(errData);

          if (errData.fatal) {
            switch (errData.type) {
              case Hls.ErrorTypes.NETWORK_ERROR:
                console.log("Network error, trying to recover...");
                hls.startLoad();
                break;
              case Hls.ErrorTypes.MEDIA_ERROR:
                console.log("Media error, attempting recovery...");
                hls.recoverMediaError();
                break;
              default:
                console.log("Cannot recover, destroying HLS instance.");
                hls.destroy();
                break;
            }
          }
        });
      } else if (player.canPlayType('application/vnd.apple.mpegurl')) {
        // Native support (e.g., in Safari)
        player.src = streamUrl;
        player.play();
      }
    // }
    is_loading = false;
  });
  
  function keyInput(event) {
    if (event.key == ' ') {
      event.preventDefault();
      if (paused) {
        player.play();
      } else {
        player.pause();
      }
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
  // document.getElementById("seeker").addEventListener("click", function(e) {
  //   let width = document.getElementById("seeker").clientWidth;
  //   time = duration * (e.offsetX / width);
  // });
  function seek(e) {
    let width = document.getElementById("seeker").clientWidth;
    time = duration * (e.offsetX / width);
  }

  // Updates library backend
  async function update_lib() {
    // if (curr_page >= imgs.length-1 && imgs.length > 0) chapter.completed = true;
    // await invoke('update_manga_lib', { item: manga });
    // let new_hist = {
    //   id: manga.id,
    //   title: manga.title,
    //   img: manga.img,
    //   plugin: manga.plugin,
    //   recent_chapter_id: chapter.id,
    //   recent_chapter_num: chapter.number,
    //   timestamp: Date.now()
    // }
    // await invoke('save_manga_history', {item: new_hist});
    // store.update(json => {
    //   json.manga_history = json.manga_history.filter(i => !(i.id == new_hist.id && i.plugin == new_hist.plugin));
    //   json.manga_history.unshift(new_hist);
    //   return json;
    // });
    goto(appHist.back());
  }

  function toggle_menu() {
    let ele_styles = document.getElementById("ep-menu").style;
    ele_styles.opacity = ele_styles.opacity == "1"? "0" : "1";
  }
  
  // window.addEventListener('resize', adjustImage);
  // function adjustImage() {
  //   const vidAspectRatio = player.videoWidth / player.videoHeight;
  //   const viewportAspectRatio = window.innerWidth / window.innerHeight;
    
  //   if (vidAspectRatio > viewportAspectRatio) {
  //     player.style.height = "auto";
  //     player.style.width = "100vw";
  //   } else {
  //     player.style.height = "100vh";
  //     player.style.width = "auto";
  //   }
  // }
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
      <button class="ep-snack-item"><Fa icon={faRightFromBracket} on:click={async () => {vlc()}} /></button>
      <button class="ep-snack-item"><Fa icon={faEllipsisVertical} /></button>
    </div>
  </div>

  <button id="show-arrow" on:click={toggle_menu}></button>

  <!-- Media Buttons -->
  <div class="ep-center">
    <button id="pause" on:click={pause}><Fa icon={paused? faPlay : faPause} /></button>
  </div>
  <div class="ep-bot">
    <div id="video-time">
      <span>{parseInt(time/60)>0? parseInt(time/60) : 0}:{(parseInt(time)%60).toString().padStart(2, '0')}</span>
      <span>{parseInt(duration/60)}:{(parseInt(duration)%60).toString().padStart(2, '0')}</span>
    </div>
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions a11y-click-events-have-key-events -->
    <progress id="seeker" value={time / duration || 0} on:click={seek}/>
  </div>
</div>

<div id="player-box">
  <!-- {#if currPlatform == "linux"} -->
  <!-- <p>Linux is not supported, vlc opening</p> -->
  <!-- {:else} -->
    <!-- svelte-ignore a11y-media-has-caption -->
    <video 
      bind:this={player}
      id="player"
      bind:currentTime={time}
      bind:duration
      bind:paused
      style="width: 100%;"
    />
  <!-- {/if} -->
  {#if error != ""}
    <p style="color: red; width: inherit; text-align: center; padding: 0; margin: 0">Error: {error}</p>
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
    color: var(--text-color);
    font-size: xx-large;
    width: 50px;
    border-radius: 25px;
    opacity: 50%;
    pointer-events: all;
  }
  .ep-bot {
    width: 100vw;
    height: 50px;
    position: absolute;
    bottom: 0;
    display: flex;
    flex-direction: column;
  }
  #video-time {
    width: 95%;
    margin-left: 25px;
    display: flex;
    gap: calc(100vw - 120px);
  }
  progress {
    width: calc(100vw - 50px);
    margin: 0 25px;
    cursor: pointer;
  }
</style>
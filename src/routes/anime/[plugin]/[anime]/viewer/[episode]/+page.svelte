<script>
  import { invoke } from "@tauri-apps/api/core";
  import { faArrowLeft, faBookmark, faEllipsisVertical } from '@fortawesome/free-solid-svg-icons';
  import { faBookmark as faOutlineBookmark } from '@fortawesome/free-regular-svg-icons';
  import { find_anime } from "$lib/anime_common.js";
  import { onMount } from 'svelte';
  import { goto } from "$app/navigation";
  import { Moon } from 'svelte-loading-spinners';
  import Fa from 'svelte-fa'
  import store from "$lib/store.js";
  import Hls from 'hls.js';
  // svelte-ignore unused-export-let TODO
  export let data;

  let anime = {};
  let episode = {};
  let is_loading = false;

  let videoElement;

  // URL to the .m3u8 file
  let streamUrl = 'https://www048.anzeat.pro/streamhls/c2039a66c65a3592163a68424f5cf8ea/ep.6.1709540076.m3u8';

  onMount(async () => {
    is_loading = true;
    // TODO: add query params for this, but if there are none do find manga
    anime = find_anime(data.plugin, data.id);
    episode = anime['episodes'][data.anime_index];
    streamUrl = await invoke('get_anime_video', { source: anime.plugin, id: episode.id });

    // Sets up link with video
    if (Hls.isSupported()) {
      const hls = new Hls();
      hls.loadSource(streamUrl);
      hls.attachMedia(videoElement);
      // todo: find placement from episode.watch_time
      hls.on(Hls.Events.MANIFEST_PARSED, () => {
        videoElement.play();
      });
    } else if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
      // Native support (e.g., in Safari)
      videoElement.src = streamUrl;
      videoElement.play();
    }
  });
</script>

<!-- svelte-ignore a11y-media-has-caption -->
<video bind:this={videoElement} controls width="600" />

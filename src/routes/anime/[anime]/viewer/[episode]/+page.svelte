<script>
  import { onMount } from 'svelte';
  import Hls from 'hls.js';

  let videoElement;
  export let data;

  // URL to the .m3u8 file
  let streamUrl = 'https://www048.anzeat.pro/streamhls/c2039a66c65a3592163a68424f5cf8ea/ep.6.1709540076.m3u8';

  onMount(() => {
    if (Hls.isSupported()) {
      const hls = new Hls();
      hls.loadSource(streamUrl);
      hls.attachMedia(videoElement);
      // hls.on(Hls.Events.MANIFEST_PARSED, () => {
      //   videoElement.play();
      // });
    } else if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
      // Native support (e.g., in Safari)
      videoElement.src = streamUrl;
      videoElement.play();
    }
  });
</script>

<!-- svelte-ignore a11y-media-has-caption -->
<video bind:this={videoElement} controls width="600" />

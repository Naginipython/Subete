<script>
  export let data;
  let videoUrl = "https://s3taku.com/abpl1245?id=NDUzODQ=&title=Akame+ga+Kill%21+Episode+6";
  let video2 = "https://sveltejs.github.io/assets/caminandes-llamigos.mp4";

  let videoElement;
  let time = 0;
	let duration;
	let paused = true;

  const playVideo = () => videoElement.play();
  const pauseVideo = () => videoElement.pause();
  const muteVideo = () => videoElement.muted = !videoElement.muted;
  function handleMousedown(e) {
	  if (paused) e.target.play();
	  else e.target.pause();
	}
</script>

{data.id}
{data.anime_index}

<div class="video-container">
    <!-- <video
        src={video2}
		on:mousedown={handleMousedown}
		bind:currentTime={time}
		bind:duration
		bind:paused
    >
        <track kind="captions" />
    </video> -->
    <!-- <div class="controls"> -->
      <!-- <button on:click={playVideo}>Play</button> -->
      <!-- <button on:click={pauseVideo}>Pause</button> -->
      <!-- <button on:click={muteVideo}>Mute/Unmute</button> -->
    <!-- </div> -->
     <iframe src={videoUrl} />
</div>

<style>
  .video-container {
    position: relative;
    max-width: 100%;
    margin: auto;
    background-color: #000;
  }
  iframe, video {
    width: 100%;
    height: auto;
  }
  /* .controls { */
    /* position: absolute; */
    /* bottom: 10px; */
    /* left: 50%; */
    /* transform: translateX(-50%); */
    /* display: flex; */
    /* justify-content: center; */
    /* gap: 10px; */
    /* background: rgba(0, 0, 0, 0.5); */
    /* padding: 5px; */
    /* border-radius: 5px; */
  /* } */
  /* button { */
    /* background: #fff; */
    /* border: none; */
    /* padding: 5px 10px; */
    /* cursor: pointer; */
    /* border-radius: 5px; */
  /* } */
</style>
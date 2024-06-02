<script>
  import { goto } from "$app/navigation"
  import { onMount } from "svelte";
  export let data;
  onMount(async () => {
        console.log("loaded")
        console.log(data.name)
    });
  function test() {
    goto("/hello/jane/jp");
  }
  console.log("changed");
</script>

<div>
  <h1>Hello, {data.name}! lang: {data.lang}</h1>
  <button on:click={test}>test</button>
</div>

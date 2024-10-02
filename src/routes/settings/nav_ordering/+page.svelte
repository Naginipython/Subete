<!-- CREDIT: https://svelte.dev/repl/827260baeeea47dc8911c2b25a4c6ce9?version=4.2.19 -->
<script>
    import { sortable } from 'svelte-agnostic-draggable'
    import mapTouchToMouseFor from 'svelte-touch-to-mouse'
    mapTouchToMouseFor('.ListItemHandle')
  
    let navLinks = ['Library', 'Updates', 'Browse', 'History', 'Settings'];
</script>

<label>Set Max: </label><input type="number" bind:value={navLinks.length}>

<div 
    class="sortable > div" 
    use:sortable={{cursor:'grabbing', handle:'.ListItemHandle', tolerance:'intersect', zIndex:10}}
    style="display:block; position:relative; width:200px; margin:20px" 
>
    {#each navLinks as link}
      <div style="display:inline-block; position:relative; width:198px; border:solid 1px black; margin:1px; padding:5px; line-height:20px;">
            {link}
            <div class="ListItemHandle"></div>
      </div>
    {/each}
</div>
  
<p style="line-height:150%">
    Please keep in mind that - while sorting - the dragged handle must be inside the sortable element in order to be recognized.
</p>

<style>
    .sortable > div {
      -webkit-touch-callout: none;
      -ms-touch-action:none; 
      touch-action:none;
      -moz-user-select:none; 
      -webkit-user-select:none; 
      -ms-user-select:none; 
      user-select:none;
    }
  
    /* For ghost space to appear when dragging */
    div:global(.ui-sortable-placeholder) { height:20px }
  
    .ListItemHandle {
      display:block; 
      position:absolute;
      top:0px; 
      right:4px; 
      width:24px; 
      height:24px;
      background-image:url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAAUklEQVRYR+3UwREAIAgDQWiXgtKuFqA+MTzOAiSzBjPMJ83zgwAIXAUkrY5yVtUxjwAzBTre/3UnazizA/wDdgHW0C5g7wAB7AL2EhIAAQR+CmweoTAhD/IaqwAAAABJRU5ErkJggg==");
      cursor:grab;
    }
</style>
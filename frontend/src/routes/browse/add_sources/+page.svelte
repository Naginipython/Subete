<script>
  import { invoke } from "@tauri-apps/api/core";
  let files;
  let json = {
    error: "",
  };
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
        test_json.hasOwnProperty("chapters_url") &&
        test_json.hasOwnProperty("get_chapters") &&
        test_json.hasOwnProperty("chapters_extra") &&
        test_json.hasOwnProperty("pages_url") &&
        test_json.hasOwnProperty("get_pages") &&
        test_json.hasOwnProperty("pages_extra")
      ) {
        json = test_json;
        media_type = test_json.media_type;
        json.error = "";
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
    delete json.error;
    switch (media_type) {
      case "manga":
        await invoke('add_manga_plugin', { newPlugin: json });
        break;
      case "ln":
        await invoke('add_ln_plugin', { newPlugin: json });
      default:
        json.error = "ERROR: media_type is invalid";
    }
    
    json.error = "";
  }
</script>
  
<!-- TODO: style much much better. Also allow for removal, or toggles -->
<!-- TODO: source specific panic button -->
<form on:submit={submit}>
    <label for="source">Upload source (.json):</label>
    <input type="file" id="source" bind:files on:change={async () => validateFile(files[0])} />
    <button type="submit" on:click={async () => submit()}>Upload</button>
    <p>{json.error}</p>
</form>
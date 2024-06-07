<script>
  import { invoke } from '@tauri-apps/api/tauri';
  let files;
  let json = {
    error: "",
  };
  
  
  async function validateFile(file) {
    const extension = file.name.split('.').pop().toLowerCase();
    if (extension !== 'json') {
      alert('Only .json files are allowed!');
      return false;
    }
    
    const text = await file.text();
    try {
      const test_json = JSON.parse(text);
      if (
        test_json.hasOwnProperty("id") &&
        test_json.hasOwnProperty("search_url") &&
        test_json.hasOwnProperty("search") &&
        test_json.hasOwnProperty("chapters_url") &&
        test_json.hasOwnProperty("get_chapters") &&
        test_json.hasOwnProperty("pages_url") &&
        test_json.hasOwnProperty("get_pages")
      ) {
        json = test_json;
        json.error = "";
      } else {
        json.error = "ERROR: Does not contain needed field(s)"
        console.log("ERROR: Does not contain needed fields");
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
    await invoke('add_plugin', { newPlugin: json });
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
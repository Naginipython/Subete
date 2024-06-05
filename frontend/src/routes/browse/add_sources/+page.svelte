<script>
    let files;
  
    function validateFile(file) {
      const extension = file.name.split('.').pop().toLowerCase();
      if (extension !== 'json') {
        alert('Only .json files are allowed!');
        return false;
      }
      // Add logic to check for specific function names (e.g., using regular expressions)
      
    }
    function submit() {
      if (!files || !validateFile(files[0])) return;
      // Replace with actual server-side logic for saving the file
      console.log('File uploaded:', files[0]);
    }
</script>
  
<form on:submit={submit}>
    <label for="source">Upload source (.json):</label>
    <input type="file" id="source" bind:files on:change={() => validateFile(files[0])} />
    <button type="submit">Upload</button>
</form>
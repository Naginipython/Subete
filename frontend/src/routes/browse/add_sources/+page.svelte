<script>
    let files;
  
    function validateFile(file) {
      const extension = file.name.split('.').pop().toLowerCase();
      if (extension !== 'js') {
        alert('Only .js files are allowed!');
        return false;
      }
      // Add logic to check for specific function names (e.g., using regular expressions)
      const pattern = /export async function search\s*\(query\)/g;
      console.log(file);
      const match = rawHtml.match(pattern);
      if (match && match[1]) {
         const jsCode = match[1];
         try {
            // retrieved = JSON.parse(jsCode);
            console.log(jsCode)
          } catch (error) {
            console.error('Error parsing JSON:', error);
          }
      }
      return true;
    }
    function submit() {
      if (!files || !validateFile(files[0])) return;
      // Replace with actual server-side logic for saving the file
      console.log('File uploaded:', files[0]);
    }
</script>
  
<form on:submit={submit}>
    <label for="source">Upload source (.js):</label>
    <input type="file" id="source" bind:files on:change={() => validateFile(files[0])} />
    <button type="submit">Upload</button>
</form>  
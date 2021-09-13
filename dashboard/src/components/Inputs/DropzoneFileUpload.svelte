<script>
  import FileUpload from "sveltefileuploadcomponent";
  let filesName = [];
  export let multiple = false;
  import uuidv1 from "uuid/v1";
  let fileid = uuidv1();
  function gotFile(file) {
    // do something with file
    filesName.push(file.detail.files.name);
    document.getElementById(fileid).innerHTML = filesName;
  }
</script>

<div class="p-5 my-2" style="border: 1px dashed #dee2e6;">
  <FileUpload let:dragging {multiple} on:input={gotFile}>
    {#if filesName.length !== 0}
      {#each filesName as file}
        <p class="w-100 mx-0">{file}</p>
      {/each}
    {:else}
      <div id={fileid}>This is{!dragging ? "n't" : ''} being dragged over.</div>
    {/if}
  </FileUpload>
</div>

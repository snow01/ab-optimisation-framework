<script>
  export let initialLabel = "Select file";

  function fileChange(e) {
    initialLabel = e.target.files[0].name;
  }

</script>

<div class="custom-file">
  <input
    type="file"
    class="custom-file-input"
    id="customFileLang"
    lang="en"
    on:change={fileChange} />
  <label class="custom-file-label" for="customFileLang">{initialLabel}</label>
</div>

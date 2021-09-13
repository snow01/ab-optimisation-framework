<script>
  import Tags from "svelte-tags-input";
  export let model;
  let tag = "";

  function handleTags(event) {
    tag = event.detail.tags;
  }
</script>

<Tags
  on:tags={handleTags}
  tags={model}
  addKeys={[9]}
  maxTags={100}
  allowPaste={true}
  allowDrop={true}
  splitWith={'/'}
  onlyUnique={true}
  removeKeys={[27]}
  placeholder={'Add new tag'}
  autoComplete={model} />

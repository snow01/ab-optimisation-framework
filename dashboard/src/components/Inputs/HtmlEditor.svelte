<script>
  import { onMount } from "svelte";
  import Quill from "quill";
  let quill = null;

  onMount(() => {
    let container = document.getElementById("editorId");
    quill = new Quill(container, {
      theme: "snow",
      modules: {
        toolbar: document.getElementById('toolbarId')
      }
    });
  });
</script>

<svelte:head>
  <link href="//cdn.quilljs.com/1.3.6/quill.snow.css" rel="stylesheet" />
</svelte:head>

<div class="quill">
  <div id="toolbarId">
    <div class="ql-formats">
      <button class="ql-bold" />
      <button class="ql-italic" />
      <button class="ql-underline" />
      <button class="ql-link" />
      <button class="ql-blockquote" />
      <button class="ql-code" />
      <button class="ql-image" />
      <button type="button" class="ql-list" value="ordered" />
      <button type="button" class="ql-list" value="bullet" />
    </div>
  </div>
  <div id="editorId" name="name" class="" ref="editor" />
</div>

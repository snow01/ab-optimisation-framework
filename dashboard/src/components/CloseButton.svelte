<script>
  export let target = ""; //'Close button target element'
  export let expanded = ""; //'Whether button is expanded (aria-expanded attribute)'
  function handleClick(evt) {
    this.$emit("click", evt);
  }
</script>

<button
  type="button"
  class="navbar-toggler"
  data-toggle="collapse"
  on:click={handleClick}
  data-target="#{target}"
  aria-controls="target"
  aria-expanded="expanded"
  aria-label="Toggle navigation">
  <span />
  <span />
</button>

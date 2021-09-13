<script>
  export let target = ""; // 'Button target element'
  export let toggled = false; // 'Whether button is toggled'
</script>

<button
  class="navbar-toggler"
  type="button"
  data-toggle="collapse"
  data-target="target"
  aria-controls="target"
  aria-expanded="toggled"
  aria-label="Toggle navigation">
  <span class="navbar-toggler-icon" />
</button>

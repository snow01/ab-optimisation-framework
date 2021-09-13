<script>
  export let type = "danger";
  export let className = "";
</script>

<div class="header {type !== "" && type !== "light" ? `bg-${type}` : ""} {className}">
  <div class="container-fluid">
    <div class="header-body">
      <slot></slot>
    </div>
  </div>
</div>

<script>
       export let active = false;
</script>

<li class="breadcrumb-item {active === true ? "active" : ""}">
       <slot></slot>
</li>
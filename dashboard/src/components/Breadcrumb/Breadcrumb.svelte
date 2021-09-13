<script>
       export let type = "";
       export let listClasses = "";
</script>

<nav aria-label="breadcrumb">
       <ol class="{listClasses} bg-{type} breadcrumb">
              <slot></slot>
       </ol>
</nav>
<script>
  // export let tag = "span";
  export let pill = false;
  export let circle = false;
  export let icon = "";
  export let variant = "default";
  export let size = "";
  export let className = "";
</script>

<span
  class="{className} {circle === true ? 'badge-circle' : ''}
  {pill === true ? 'badge-pill' : ''}
  {variant !== 'default' ? `badge-${variant}` : ""} badge"
  {size}>
  <slot>
    {#if icon}
      <i class={icon} />
    {/if}
  </slot>
</span>

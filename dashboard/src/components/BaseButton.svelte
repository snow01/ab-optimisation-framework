<script>
  import { createEventDispatcher } from "svelte";

  export let className = "";
  export let type = "default" //  Button type (primary|secondary|danger etc)
  export let loading = false;
  export let round = false;
  export let wide = false;
  export let icon = false;
  export let block = false;
  export let nativeType = "button"; // Button native type (e.g button, input etc)
  export let disabled = false;
  export let size = ""; // Button size (sm|lg)
  export let outline = false; // Whether button is outlined (only border has color)
  export let link = false; // Whether button is a link (no borders or background)
  const dispatch = createEventDispatcher();

  const Clicked = () => {
    dispatch("click");
  };
  
</script>

<button 
       style="{block ? "width:100%;" : ""}" 
       type="{nativeType}"
       disabled="{disabled || loading}"
       size="{size}"
       variant="{!outline ? type : `outline-${type}`}"
       on:click={Clicked}
       class="btn {block ? `btn-${block}` : ""} {outline ? "" : type ? `btn-${type}` : ""} {className} {round ? 'rounded-circle' : ""} {wide ? 'btn-wd' : ""} {icon ? "btn-icon btn-fab" : ""} {link ? 'btn-link' : ""} btn-{size} {outline === false ? type : `btn-outline-${type}`}"
       >
  <slot name="loading">
    {#if loading}
      <i class="fas fa-spinner fa-spin"></i>
    {/if}
  </slot>
  <slot></slot>
</button>

<style>
  .base-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .base-button  i {
      padding: 0 3px;
    }
</style>
<script>
  export let className = "";
  export let headerClasses = "";
  export let bodyClasses = "";
  export let footerClasses = "";

  export let gradient = "";
  export let shadow = "";
  export let shadowSize = "";
  export let type = "";
  export let hover = false;
  export let imgTop = false;
  export let imgBottom = false;
  export let imgSrc = "";
  export let alt = "";
  export let background = "";

  export let noBody = false; //Whether card should have wrapper body class

  let slots = $$props.$$slots;
</script>

<style>
  .bg-candy {
    background: #262A33!important;
  }
</style>

<div
  class="{className}
  {gradient !== '' ? `bg-gradient-${gradient}` : ''}
  {shadow ? `shadow` : ''}
  {type !== '' ? `bg-${type}` : ''}
  {shadowSize !== '' ? `shadow-${shadowSize}` : ''}
  {hover ? `card-lift--hover` : ''} {background === "true" ? "bg-candy" : ""} card">
  {#if imgTop === false && imgBottom === false}
    {#if imgSrc !== ""}
      <img src={imgSrc} alt={alt} class="card-img">
    {/if}
  {/if}
  {#if imgTop !== false}
    <slot name="image">
      <img {alt} src={imgSrc} class="card-img-top" />
    </slot>
  {/if}
  {#if slots.header}
    <div class="card-header  {headerClasses}">
      <slot name="header" />
    </div>
  {/if}
  {#if !noBody}
    <div class="card-body {bodyClasses}">
      <slot />
    </div>
  {/if}
  {#if noBody}
    <slot />
  {/if}
  {#if slots.footer}
    <div class="card-footer {footerClasses}">
      <slot name="footer" />
    </div>
  {/if}
  {#if imgBottom !== false}
    <slot name="image">
      <img {alt} src={imgSrc} class="card-img-bottom" />
    </slot>
  {/if}
</div>

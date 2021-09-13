<script>
  import Card from "./Card.svelte";
  export let type = "primary";
  export let icon = "";
  export let title = "";
  export let subTitle = "";
  export let iconClasses = "";
  export let className = "";
</script>

<Card className="card-stats card {className}" show-footer-line="true">
  <div class="row">
    <div class="col">
      <slot>
        {#if title}
          <h5 class="card-title text-uppercase text-muted mb-0">{title}</h5>
        {/if}
        {#if subTitle}
          <span class="h2 font-weight-bold mb-0">{subTitle}</span>
        {/if}
      </slot>
    </div>
    {#if icon}
      <div class="col-auto">
        <slot name="icon">
          <div
            class="icon icon-shape text-white reounded-circle shadow {`bg-${type}`}
            {iconClasses}">
            <i class={icon} />
          </div>
        </slot>
      </div>
    {/if}
  </div>

  <p class="mt-3 mb-0 text-sm">
    <slot name="footer" />
  </p>
</Card>

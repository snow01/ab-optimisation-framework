<script>
  import { fade } from "svelte/transition";
  export let type = "dafault";
  export let dismissible = false;
  export let icon = "";
  export let notifyClassNames = "";
  let visible = true;
  let show = true;
  export let dataNotify = false;
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  const removeNotify = () => {
    dispatch("remove");
  };

  function dismissAlert() {
    visible = false;
  }
</script>

<div transition:fade>
  {#if visible === true}
    <div
      model={visible}
      data-notify={dataNotify === true ? 'container' : ''}
      variant={type}
      class="alert alert-{type}
      {dismissible === true ? 'alert-dismissible' : ''}
      {dataNotify === true ? 'alert-notify' : ''}
      {notifyClassNames}"
      role="alert">
      {#if !dismissible}
        <slot />
      {:else}
        {#if icon || $$props.$$icon}
          <slot name="icon">
            <span class="alert-icon" dat-notify="icon">
              <i class={icon} />
            </span>
          </slot>
        {/if}

        <span class="alert-text">
          <slot />
        </span>

        <slot name="dismiss-icon">
          <button
            type="button"
            class="close"
            data-dismiss="alert"
            aria-label="Close"
            on:click={dismissAlert}>
            {#if !dismissible}
              <span aria-hidden="true" on:click={dismissAlert()}>×</span>
            {:else}
              <span aria-hidden="true">×</span>
            {/if}
          </button>
        </slot>
      {/if}
    </div>
  {/if}

</div>

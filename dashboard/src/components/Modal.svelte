<script>
  import { clickOutside } from "./clickOutside.js";
  import { fly, fade } from "svelte/transition";
  export let show = "";
  export let showClose = true;
  export let type = "";
  type = validatorType(type);
  export let modalClasses = "";
  export let size = "";
  size = validatorSize(size);
  export let modalContentClasses = "";
  export let gradient = "";
  export let headerClasses = "";
  export let bodyClasses = "";
  export let footerClasses = "";
  export let animationDuration = 500;
  let textWhite = false;

  if (gradient !== "") {
    textWhite = true;
  }
  let slots = $$props.$$slots;

  function validatorType(type) {
    let acceptedValues = ["", "notice", "mini"];
    return acceptedValues.indexOf(type) !== -1;
  }
  function validatorSize(size) {
    let acceptedValues = ["", "sm", "lg"];
    if (acceptedValues.indexOf(size) !== -1) {
      return size;
    }
    return false;
  }

  function handleClickOutside(event) {
    closeModal();
  }

  function closeModal() {
    show = false;
  }

  function showModal() {
    show = true;
  }

  if (show === true) {
    document.body.classList.add("modal-show");
  } else {
    document.body.classList.remove("modal-show");
  }
</script>

<style>
  .modal-backdrop {
    background-color: rgba(0, 0, 0, 0.6) !important;
  }
</style>

{#if show === true}
  <div>
    <div
      use:clickOutside
      on:click_outside={handleClickOutside}
      in:fly={{ y: -100, duration: animationDuration }}
      out:fly={{ y: -100, duration: animationDuration }}
      class="modal show"
      ref="app-modal"
      {size}
      hide-header={!slots.header}
      modal-class="{type === 'mini' ? 'modal-mini' : ''}
      {modalClasses}"
      tabindex="-1"
      role="dialog"
      centered
      on:close={closeModal}
      on:hide={closeModal}
      header-class={headerClasses}
      footer-class={footerClasses}
      content-class="{gradient ? `bg-gradient-${gradient}` : ''}
      {modalContentClasses}"
      body-class={bodyClasses}
      aria-hidden={!show}
      style="display: block;">
      <div
        class="modal-dialog modal-dialog-centered {size === "sm" ? "modal-sm" : "modal-md"}">
        <div
          class="modal-content {gradient ? `bg-gradient-${gradient}` : ''}
          {modalContentClasses}">
          {#if slots.header}
            <header class="modal-header {headerClasses}">
              <slot name="header" />
              <slot name="close-button">
                {#if showClose}
                  <button
                    type="button"
                    class="close"
                    on:click={closeModal}
                    data-dismiss="modal"
                    aria-label="Close">
                    <span
                      aria-hidden={!show}
                      class={textWhite === true ? 'text-white' : ''}>
                      ×
                    </span>
                  </button>
                {/if}
              </slot>
            </header>
          {/if}
          <div class="modal-body {bodyClasses}">
            <slot />
          </div>

          {#if slots.footer}
            <footer class="modal-footer {footerClasses}">
              <slot name="footer" />
            </footer>
          {/if}
        </div>
      </div>
    </div>
  </div>
  <div class="modal-backdrop" />
{/if}

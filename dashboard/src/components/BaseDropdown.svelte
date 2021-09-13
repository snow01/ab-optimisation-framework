<script>
  export let tag = "div";
  export let titleTag = "button";
  export let title = "";
  export let tagClasses = "";
  export let direction = "down";
  export let icon = "";
  export let titleClasses = "";
  export let menuClasses = "";
  export let menuOnRight = "";
  export let hasToggle = true;
  export let isOpen = true;
  import { clickOutside } from "./clickOutside.js";
  import { fly } from "svelte/transition";

  function toggleDropdown() {
    if (isOpen === true) {
      isOpen = false;
    } else {
      isOpen = true;
    }
  }

  function handleClickOutside(event) {
    closeDropDown();
  }

  function closeDropDown() {
    isOpen = false;
  }
</script>

<style>
  .dropdown {
    cursor: pointer;
    user-select: none;
  }
  .btn-rotate:hover {
    cursor: pointer;
  }
</style>

{#if tag === 'li' && titleTag === 'a'}
  <li
    use:clickOutside
    on:click_outside={handleClickOutside}
    class="{isOpen === true ? 'show' : ''} drop{direction} {tagClasses}"
    on:click={toggleDropdown}>
    <slot name="title-container" is-open={isOpen}>
      <a
        class="btn-rotate {hasToggle === true ? 'dropdown-toggle' : ''}
        {titleClasses}"
        href="#!"
        aria-expanded={isOpen}
        data-toggle="dropdown">
        <slot name="title" is-open={isOpen}>
          <i class={icon} />
          {title}
        </slot>
      </a>
    </slot>
    {#if isOpen === true}
      <ul
        class="dropdown-menu show {menuOnRight ? 'dropdown-menu-right' : ''}
        {menuClasses}"
        >
        <slot />
      </ul>
    {/if}
  </li>
{:else}
  <div
    use:clickOutside
    on:click_outside={handleClickOutside}
    class="{isOpen === true ? 'show' : ''} drop{direction} {tagClasses}"
    on:click={toggleDropdown}>
    <slot name="title-container" is-open={isOpen}>
      <button
        class="btn-rotate {hasToggle === true ? 'dropdown-toggle' : ''}
        {titleClasses}"
        aria-expanded={isOpen}
        data-toggle="dropdown">
        <slot name="title" is-open={isOpen}>
          <i class={icon} />
          {title}
        </slot>
      </button>
    </slot>
    {#if isOpen === true}
      <ul
        class="dropdown-menu show {menuOnRight ? 'dropdown-menu-right' : ''}
        {menuClasses}"
        >
        <slot />
      </ul>
    {/if}
  </div>
{/if}

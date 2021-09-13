<script>
  export let show = false;
  export let transparent = false;
  export let expand = "lg";
  export let menuClasses = "";
  export let topClasses = "";
  export let containerClasses = "container";
  export let type = "";
  export let position = "";
  let hasMenu = false;
  type = validator(type);
  function validator(value) {
    if (
      [
        "",
        "dark",
        "success",
        "danger",
        "warning",
        "white",
        "primary",
        "light",
        "info",
        "vue"
      ].includes(value)
    ) {
      return value;
    } else return "";
  }
  let color = `bg-${type}`;
  if ($$props.$$slots.default) {
    hasMenu === true;
  }
  function toggleMenu() {
    show = !show;
  }
  function closeMenu() {
    show = false;
  }
</script>

<!-- {!transparent ? `bg-${type}` : ''} -->

<nav
  toggleable
  class="navbar-expand-{expand}
  {transparent !== false ? 'navbar-transparent' : ''}
  {!position ? `navbar-${position}` : ''}
  {type !== '' ? `navbar-${type}` : ''}
  {topClasses} navbar">
  <div class={containerClasses}>
    <slot name="brand" />

    <slot name="toggle-button">
      {#if hasMenu}
        <button
          class="navbar-toggler collapsed"
          type="button"
          on:click={toggleMenu}
          aria-expanded="false"
          aria-label="Toggle navigation">
          <span class="navbar-toggler-bar navbar-kebab" />
          <span class="navbar-toggler-bar navbar-kebab" />
          <span class="navbar-toggler-bar navbar-kebab" />
        </button>
      {/if}
    </slot>

    <button
      type="button"
      aria-label="Toggle navigation"
      data-target="#nav-text-collapse"
      on:click={toggleMenu}
      aria-controls="nav-text-collapse"
      aria-expanded="false"
      class="navbar-toggler">
      <span class="navbar-toggler-icon" />
    </button>

    {#if show !== false}
      <div
        id="nav-text-collapse"
        class="navbar-custom-collapse collapse show {menuClasses}
        {show}"
        visible={show}
        on:clickoutside={closeMenu}>
        <slot close-menu={closeMenu} />
      </div>
    {/if}

  </div>
</nav>

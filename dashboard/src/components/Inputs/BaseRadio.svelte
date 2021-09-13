<script>
  export let name = "";
  export let disabled = "";
  export let inline = "";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  const Clicked = () => {
    dispatch("click");
  };
  let id = Math.random()
    .toString(16)
    .slice(2);
  name = name.toLowerCase();

  function updateRadio(e) {
    name = !name;
    Clicked();
  }
</script>

<div
  class="custom-control custom-radio {inline !== '' ? 'form-check-inline' : ''}
  {disabled !== '' ? `${disabled}` : ''}">
  <input
    {id}
    class="custom-control-input"
    type="radio"
    {disabled}
    on:click={updateRadio}
    checked={name === true || name === "checked" ? true : false} />
  <label for={id} class="custom-control-label">
    <slot>
      {#if inline}
        <span>&nbsp;</span>
      {/if}
    </slot>
  </label>
</div>

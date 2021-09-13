<script>
  export let disabled = "";
  export let inline = "";
  export let inputClasses = "";
  export let type = "";
  export let model = "";
  export let className = "";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  const Clicked = () => {
    dispatch("click");
  };
  let id = Math.random()
    .toString(6)
    .slice(2);
  function updateCheckBox(e) {
    model = !model;
    Clicked();
  }
</script>

<div
  class="custom-control custom-checkbox {disabled ? disabled : ''}
  {type !== '' ? `custom-checkbox-${type}` : ''}
  {inline !== '' ? 'form-check-inline' : ''}
  {className}">
  <input
    type="checkbox"
    on:click={updateCheckBox}
    {id}
    class="custom-control-input {inputClasses !== '' ? `${inputClasses}` : ''}"
    {disabled}
    checked={model === true ? true : false} />
  <label for={id} class="custom-control-label">
    <slot>
      {#if inline}
        <span>&nbsp;</span>
      {/if}
    </slot>
  </label>
</div>

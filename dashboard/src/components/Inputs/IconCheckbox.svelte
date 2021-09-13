<script>
  export let checked = false;
  export let name;
  export let title;
  export let icon;
  export let disabled;

  function updateValue() {
    cheched = !cheched;
  }
</script>

<div
  class="choice {checked === true ? 'active' : ''}"
  data-toggle="wizard-checkbox"
  on:clicl={updateValue}>
  <input type="checkbox" name={name} disabled={disabled} checked="checked" />
  <div class="icon">
    <slot name="icon">
      <i class={icon} />
    </slot>
  </div>
  <slot name="title">
    <h6>{ title }</h6>
  </slot>
</div>

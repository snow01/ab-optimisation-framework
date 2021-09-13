<script>
  export let required = "";
  export let group = "";
  export let alternative = "";
  export let label = "";
  export let error = "";
  export let successMessage = "";
  export let labelClasses = "form-control-label";
  export let inputClasses = "";
  export let inputGroupClasses = "";
  export let value = "";
  export let type = "";
  export let appendIcon = "";
  export let prependIcon = "";
  export let rules = "";
  export let name = "";
  export let attrs = $$props.$$attrs;
  export let placeholder = "";
  export let id = "";
  let appendSlot;
  let prependSlot;
  if ($$props.$$slots !== undefined) {
    if ($$props.$$slots.append) {
      appendSlot = $$props.$$slots.append;
    }
    if ($$props.$$slots.prepend) {
      prependSlot = $$props.$$slots.prepend;
    }
  }
  // export let baseRequired = "";
  let focused = false;

  function listeners() {
    return {
      ...$$props.$$listeners,
      input: updateValue,
      focus: onFocus,
      blur: onBlur
    };
  }
  function slotData() {
    return {
      focused: focused,
      error: error,
      ...$$props.$$listeners
    };
  }
  function updateValue(e) {
    value = e.target.value;
    // this.$emit("input", value);
  }
  function onFocus(e) {
    focused = true;
    // this.$emit("focus", e);
  }
  function onBlur(e) {
    focused = false;
    if (e.target.value === "") {
      if (required === true) {
        error = `The ${name} field is required`;
      }
    } else {
      error = "";
    }
    // this.$emit("blur", e);
  }
</script>

<div {rules} {name} bind={$attrs}>
  <div class="form-group">
    <slot name="label">
      {#if label !== ''}
        <label class={labelClasses}>{label}</label>
      {/if}
    </slot>
    <div
      class="{$$props.append !== undefined || $$props.prepend !== undefined || appendIcon !== '' || prependIcon !== '' || group !== '' ? 'input-group' : ''}
      {focused === true ? 'focused' : ''}
      {alternative !== '' ? 'input-group-alternative' : ''}
      {label !== '' || $$props.label !== undefined ? 'has-label' : ''}
      {inputGroupClasses}
      ">
      {#if prependSlot !== undefined}
        <div class="input-group-prepend">
          <span class="input-group-text">
            <slot name="prepend" />
          </span>
        </div>
      {/if}
      {#if prependIcon !== '' || $$props.prepend !== undefined}
        <div class="input-group-prepend">
          <span class="input-group-text">
            <slot name="prepend">
              <i class={prependIcon} />
            </slot>
          </span>
        </div>
      {/if}
      <slot bind={slotData}>
        <input
          {value}
          {id}
          {type}
          {required}
          on:focus={onFocus}
          on:blur={onBlur}
          on:change={updateValue}
          bind={attrs}
          {placeholder}
          class="form-control {inputClasses}" />
      </slot>
      {#if appendIcon !== '' || $$props.append !== undefined}
        <div class="input-group-append">
          <span class="input-group-text">
            <slot name="append">
              <i class={appendIcon} />
            </slot>
          </span>
        </div>
      {/if}
      {#if appendSlot !== undefined}
        <div class="input-group-append">
          <span class="input-group-text">
            <slot name="append" />
          </span>
        </div>
      {/if}
      <slot name="infoBlock" />
    </div>
    <slot name="success">
      <div class="valid-feedback">
        {#if error === '' && successMessage !== ''}{error}{/if}
      </div>
    </slot>
    <slot name="error">
      <div class={error ? 'invalid-feedback d-block' : ''}>
        {#if error}{error}{/if}
      </div>
    </slot>
  </div>
</div>

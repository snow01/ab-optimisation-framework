<script>
  export let options = []; //'Radio options. Should be an array of objects {value: "", label: ""}',
  export let value = ""; //'Radio value'
  export let buttonClasses = ""; //'Inner button css classes'
  let model = {
    prop: "value",
    event: "change"
  };

  function get() {
    return value;
  }

  function set(val) {
    this.$emit("change", val);
  }
</script>

<div class="btn-group-toggle" data-toggle="buttons">
  {#each options as option, index}
    <label
      for=""
      key={index}
      class="btn {value === option.value ? 'active' : ''}
      {buttonClasses}">
      <input
        type="radio"
        id="option1"
        {model}
        value={option.value}
        autocomplete="off"
        checked="" />
      {option.value}
    </label>
  {/each}
</div>

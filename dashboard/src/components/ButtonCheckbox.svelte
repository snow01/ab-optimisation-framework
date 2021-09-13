<script>
  export let value = ""; //'Checked value'
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
       <label for="" class="btn {value ? "active" : ''} {buttonClasses}">
              <input type="checkbox" checked="" autocomplete="off" model="model">
              <slot></slot>
       </label>
</div>

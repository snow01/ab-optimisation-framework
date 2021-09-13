<script>
  // export let value = [];
  export let type = "";
  export let onText = "Yes";
  export let offText = "No";
  let baseClass = "custom-toggle-";
  export let model = "";
  if (type) {
    baseClass = baseClass + type;
  }

  function triggerToggle() {
    model != model;
  }
</script>

<label class="custom-toggle {baseClass}">
  <input type="checkbox" checked={model} />
  <span
    class="custom-toggle-slider rounded-circle"
    data-label-off={offText}
    data-label-on={onText} />
</label>

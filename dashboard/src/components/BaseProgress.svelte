<script>
  export let striped = ""; //"Whether progress is striped"
  export let animated = "";
  //export let description = ""; //"Whether progress is animated (works only with `striped` prop together)"
  export let label = ""; //"Progress label (shown on the left above progress)"
  export let height = 3; //"Progress line height"
  export let type = "default"; //"Progress type (e.g danger, primary etc)"
  export let showLabel = false;
  export let progressClasses = ""; //'Progress css classes'
  export let size = "";
  export let value = 0; //"Progress value"
  value = validator(value);
  function validator(value) {
    if (value >= 0 && value <= 100) {
      return value;
    } else return false;
  }
</script>

<div style="width: 100%; height:3px;">
  <div class="wrapper {progressClasses}">
    {#if showLabel}
      <div class={`progress-${type}`}>
        <div class="progress-label">
          <slot name="label">
            <span>{label}</span>
          </slot>
        </div>
        <div class="progress-percentage">
          <slot>
            <span>{value}%</span>
          </slot>
        </div>
      </div>
    {/if}
    <div
      class="progress"
      {size}
      style="height: {height}px">
      <div
        class="progressbar {striped ? 'progress-bar-striped' : ''}
        {animated ? 'progress-bard-animated' : ''}
        {type ? `bg-${type}` : ''}"
        {value} aria-valuemin="0" aria-valuemax="100" aria-valuenow={value} style="width: {value}%;" />
    </div>
  </div>
</div>

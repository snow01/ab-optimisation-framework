<script>
  export let badgeType = "success";
  export let badgeIcon = "";
  export let inverted = "";
  let title = "";
</script>

<div class="timeline-block {inverted === true ? 'timeline-inverted' : ''}">
  <slot name="badge">
    <span class="timeline-step {`badge-${badgeType}`}">
      <i class={badgeIcon} />
    </span>
  </slot>
  <div class="timeline-content">
    <slot />
  </div>
</div>

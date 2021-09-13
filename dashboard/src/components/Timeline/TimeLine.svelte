<script>
       export let type = '';
</script>

<div class="timeline {type ? `timeline-${type}` : ""}">
       <slot></slot>
</div>
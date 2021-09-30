<script>
	import BaseButton from '$components/BaseButton.svelte';

	export let icon="";
	export let title="";
</script>

<BaseButton type="neutral" className="el-tooltip" icon size="sm">
            <span class="btn-inner--icon">
              <i class="fas fa-{icon}" />
            </span>
	<span class="btn-inner--text">{title}</span>
</BaseButton>
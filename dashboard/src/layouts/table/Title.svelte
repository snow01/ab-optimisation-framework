<script>
	export let title="TODO";
</script>

<div class="card-header border-0">
	<div class="row">
		<div class="col-6">
			<h3 class="mb-0">{title}</h3>
		</div>
		<div class="col-6 text-right">
			<slot></slot>
		</div>
	</div>
</div>
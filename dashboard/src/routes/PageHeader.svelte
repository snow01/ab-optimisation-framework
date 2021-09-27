<script>
	import RouteBreadCrumb from '$components/Breadcrumb/RouteBreadcrumb.svelte';
	import BaseHeader from '$components/BaseHeader.svelte';

	export let name = '';
</script>

<BaseHeader className='pb-3'>
	<div class='row align-items-center py-4'>
		<div class='col-lg-6 col-7'>
			<h6 class='h2 text-white d-inline-block mb-0'>Applications</h6>
			<nav aria-label='breadcrumb' class='d-none d-md-inline-block ml-md-4'>
				<RouteBreadCrumb {name} />
			</nav>
		</div>
		<slot name='header'></slot>
	</div>
</BaseHeader>
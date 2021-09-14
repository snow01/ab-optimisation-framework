<!--<script>-->
<!--	import { fade } from 'svelte/transition';-->

<!--	// Components-->
<!--	import RouteBreadCrumb from '$components/Breadcrumb/RouteBreadcrumb.svelte';-->
<!--	import BaseHeader from '$components/BaseHeader.svelte';-->

<!--	export let name = 'Dashboard';-->
<!--	import { onMount } from 'svelte';-->

<!--	onMount(function() {-->
<!--		let topmenu = document.getElementsByTagName('nav').item(0);-->
<!--		topmenu.classList.add('bg-danger');-->
<!--		topmenu.classList.add('navbar-dark');-->
<!--		topmenu.classList.remove('navbar-light');-->
<!--		let search = document.getElementsByTagName('form').item(0);-->
<!--		search.classList.remove('navbar-search-dark');-->
<!--		search.classList.add('navbar-search-light');-->
<!--	});-->

<!--</script>-->

<!--<div transition:fade={{ duration: 250 }}>-->
<!--	<BaseHeader className='pb-3'>-->
<!--		<div class='row align-items-center py-4'>-->
<!--			<div class='col-lg-6 col-7'>-->
<!--				<h6 class='h2 text-white d-inline-block mb-0'>Applications</h6>-->
<!--				<nav aria-label='breadcrumb' class='d-none d-md-inline-block ml-md-4'>-->
<!--					<RouteBreadCrumb {name} />-->
<!--				</nav>-->
<!--			</div>-->
<!--		</div>-->
<!--	</BaseHeader>-->
<div class='container-fluid pt-3'>
	<div class='row'>
		<h1>Welcome to SvelteKit</h1>
	</div>
	<div class='row'>
		<p>Visit <a href='https://kit.svelte.dev'>kit.svelte.dev</a> to read the documentation</p>
	</div>
</div>
<!--</div>-->

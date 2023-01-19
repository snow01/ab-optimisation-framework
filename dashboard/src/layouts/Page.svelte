<script>
	import '$assets/global.css';
	import { fade } from 'svelte/transition';
	import DashboardNavbar from './DashboardNavbar.svelte';

	// Components
	import SideBar from '$components/SidebarPlugin/SideBar.svelte';
	import SideBarItem from '$components/SidebarPlugin/SideBarItem.svelte';

	export let name = '';
	export let type = 'default';

	import { onMount } from 'svelte';
	import PageHeader from './PageHeader.svelte';
	import ContentFooter from '$layouts/ContentFooter.svelte';

	onMount(function() {
		let topmenu = document.getElementsByTagName('nav').item(0);
		topmenu.classList.add('bg-danger');
		topmenu.classList.add('navbar-dark');
		topmenu.classList.remove('navbar-light');
		let search = document.getElementsByTagName('form').item(0);
		search.classList.remove('navbar-search-dark');
		search.classList.add('navbar-search-light');
	});

	// export let currentRoute;
	let toggle = false;
	let sideBarToggle = true;

	function changeToggle() {
		if (document.body.classList.contains('g-sidenav-hidden')) {
			toggle = false;
			sideBarToggle = false;
		} else {
			toggle = true;
			sideBarToggle = true;
			if (window.innerWidth < 1200) {
				let collapse = document.getElementsByClassName('sidebarcollapse');
				for (var i = 0; i < collapse.length; i++) {
					collapse[i].classList.remove('d-none');
				}
			}
		}
	}

	onMount(async () => {
		if (window.innerWidth < 1200) {
			toggle = false;
			sideBarToggle = false;
			document.body.classList = 'g-sidenav-hidden';
			let collapse = document.getElementsByClassName('sidebarcollapse');
			for (let i = 0; i < collapse.length; i++) {
				collapse[i].classList.remove('d-none');
			}
		} else {
			toggle = true;
			sideBarToggle = true;
			document.body.classList = 'g-sidenav-pinned g-sidenav-show';
		}
	});

	let activeSidebarItems = [
		{ dashboard: true },
		{ examples: false },
		{ components: false },
		{ forms: false },
		{ tables: false },
		{ maps: false },
		{ widgets: false },
		{ charts: false },
		{ calendar: false }
	];

</script>

<div class='wrapper'>
	<!-- sidebar component  -->
	<SideBar toggle={sideBarToggle} on:click={changeToggle}>
		<ul class='navbar-nav' slot='links' id='navbar-nav'>
			<SideBarItem
				link={{ name: 'Dashboards', icon: 'ni ni-shop text-primary', children: 2, isActive: activeSidebarItems[0].dashboard }}>
				<SideBarItem
					link={{ name: 'Dashboard', path: '/dashboard', children: 0 }} />
				<SideBarItem
					link={{ name: 'Alternative', path: '/dashboard/alternative', children: 0 }} />
			</SideBarItem>

			<SideBarItem
				link={{ name: 'Examples', icon: 'ni ni-ungroup text-orange', children: 6, isActive: activeSidebarItems[1].examples }}>
				<SideBarItem
					link={{ name: 'Pricing', path: '/pricing', children: 0 }} />
				<SideBarItem link={{ name: 'Login', path: '/login', children: 0 }} />
				<SideBarItem
					link={{ name: 'Register', path: '/register', children: 0 }} />
				<SideBarItem link={{ name: 'Lock', path: '/lock', children: 0 }} />
				<SideBarItem
					link={{ name: 'Timeline', path: '/dashboard/pages/timeline', children: 0 }} />
				<SideBarItem
					link={{ name: 'Profile', path: '/dashboard/pages/user', children: 0 }} />
			</SideBarItem>

			<SideBarItem
				link={{ name: 'Components', icon: 'ni ni-ui-04 text-info', children: 7, isActive: activeSidebarItems[2].components }}>
				<SideBarItem
					link={{ name: 'Buttons', path: '/dashboard/components/buttons', children: 0 }} />
				<SideBarItem
					link={{ name: 'Cards', path: '/dashboard/components/cards', children: 0 }} />
				<SideBarItem
					link={{ name: 'Grid', path: '/dashboard/components/grid-system', children: 0 }} />
				<SideBarItem
					link={{ name: 'Notifications', path: '/dashboard/components/notifications', children: 0 }} />
				<SideBarItem
					link={{ name: 'Icons', path: '/dashboard/components/icons', children: 0 }} />
				<SideBarItem
					link={{ name: 'Typography', path: '/dashboard/components/typography', children: 0 }} />

				<SideBarItem link={{ name: 'Multi Level', children: 3 }}>
					<SideBarItem
						link={{ name: 'Third level menu', path: '#!', children: 0 }} />
					<SideBarItem
						link={{ name: 'Just another link', path: '#!', children: 0 }} />
					<SideBarItem
						link={{ name: 'One last link', path: '#!', children: 0 }} />
				</SideBarItem>
			</SideBarItem>

			<SideBarItem
				link={{ name: 'Forms', icon: 'ni ni-single-copy-04 text-pink', children: 3, isActive: activeSidebarItems[3].forms }}>
				<SideBarItem
					link={{ name: 'Elements', path: '/dashboard/forms/elements', children: 0 }} />
				<SideBarItem
					link={{ name: 'Components', path: '/dashboard/forms/components', children: 0 }} />
				<SideBarItem
					link={{ name: 'Validation', path: '/dashboard/forms/validation', children: 0 }} />
			</SideBarItem>

			<SideBarItem
				link={{ name: 'Tables', icon: 'ni ni-align-left-2 text-default', children: 3, isActive: activeSidebarItems[4].tables }}>
				<SideBarItem
					link={{ name: 'Tables', path: '/dashboard/tables/regular', children: 0 }} />
				<SideBarItem
					link={{ name: 'Sortable', path: '/dashboard/tables/sortable', children: 0 }} />
				<SideBarItem
					link={{ name: 'Paginated Tables', path: '/dashboard/tables/paginated', children: 0 }} />
			</SideBarItem>

			<SideBarItem
				link={{ name: 'Maps', icon: 'ni ni-map-big text-primary', children: 2, isActive: activeSidebarItems[5].maps }}>
				<SideBarItem
					link={{ name: 'Google', path: '/dashboard/maps/google', children: 0 }} />
				<SideBarItem
					link={{ name: 'Vector', path: '/dashboard/maps/vector', children: 0 }} />
			</SideBarItem>
			<SideBarItem
				link={{ name: 'Widgets', icon: 'ni ni-archive-2 text-green', path: '/dashboard/widgets', children: 0, single: true, isActive: activeSidebarItems[6].widgets }} />
			<SideBarItem
				link={{ name: 'Charts', icon: 'ni ni-chart-pie-35 text-info', path: '/dashboard/charts', children: 0, single: true, isActive: activeSidebarItems[7].charts }} />

			<SideBarItem
				link={{ name: 'Calendar', icon: 'ni ni-calendar-grid-58 text-red', path: '/dashboard/calendar', children: 0, single: true, isActive: activeSidebarItems[8].calendar }} />
		</ul>
		<div slot='links-after'>
			<hr class='my-3' />
			<h6 class='navbar-heading p-0 text-muted'>Documentation</h6>
			<ul class='nav navbar-nav mb-md-3'>
				<li class='nav-item'>
					<a
						rel='noopener'
						target='_blank'
						href='https://www.creative-tim.com/learning-lab/svelte/overview/argon-dashboard'
						class='nav-link'>
						<i class='ni ni-spaceship'></i>
						<span class='navbar-text p-0'>Getting started</span>
					</a>
				</li>
				<li class='nav-item'>
					<a
						rel='noopener'
						target='_blank'
						href='https://www.creative-tim.com/learning-lab/svelte/colors/argon-dashboard'
						class='nav-link'>
						<i class='ni ni-palette'></i>
						<span class='navbar-text p-0'>Foundation</span>
					</a>
				</li>
				<li class='nav-item'>
					<a
						rel='noopener'
						target='_blank'
						href='https://www.creative-tim.com/learning-lab/svelte/avatar/argon-dashboard'
						class='nav-link'>
						<i class='ni ni-ui-04'></i>
						<span class='navbar-text p-0'>Components</span>
					</a>
				</li>
				<li class='nav-item'>
					<a
						rel='noopener'
						target='_blank'
						href='https://www.creative-tim.com/learning-lab/svelte/charts/argon-dashboard'
						class='nav-link'>
						<i class='ni ni-chart-pie-35'></i>
						<span class='navbar-text p-0'>Plugins</span>
					</a>
				</li>
			</ul>
		</div>
	</SideBar>
	<div class='main-content'>
		<DashboardNavbar {type} showSidebar={toggle} on:click={changeToggle} />
		<div transition:fade={{ duration: 250 }}>
			<PageHeader {name}>
				<slot name='header'></slot>
			</PageHeader>
			<!-- your content here -->
			<slot name='content'></slot>
		</div>
	</div>
</div>

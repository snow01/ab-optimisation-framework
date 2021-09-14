<script>
	import BaseNav from '$components/Navbar/BaseNav.svelte';
	// import Modal from '$components/Modal.svelte';
	import BaseDropdown from '$components/BaseDropdown.svelte';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	const Clicked = () => {
		dispatch('click');
	};

	export let type = 'default';

	let activeNotifications = false;
	let showMenu = false;
	let searcModalVisible = false;
	let searchQuery = '';
	export let showSidebar = false;

	import { onMount } from 'svelte';

	onMount(async () => {
		if (document.body.classList.contains('g-sidenav-hidden')) {
			showSidebar = false;
		} else {
			showSidebar = true;
		}

		window.addEventListener('resize', function() {
			if (document.body.classList.contains('g-sidenav-hidden')) {
				showSidebar = false;
			} else {
				showSidebar = true;
			}
		});
	});

	function capitalizeFirstLetter(string) {
		return string.charAt(0).toUpperCase() + string.slice(1);
	}

	function toggleNotificationDropDown() {
		activeNotifications = !activeNotifications;
	}

	function closeDropDown() {
		activeNotifications = false;
	}

	function displaySidebar() {
		showSidebar = !showSidebar;
		if (showSidebar === true) {
			if (document.body.classList.contains('g-sidenav-hidden')) {
				document.body.classList = 'g-sidenav-pinned g-sidenav-show';
			}
		} else {
			document.body.classList = 'g-sidenav-hidden';
		}
	}

	function toggleSidebar() {
		displaySidebar();
		Clicked();
	}

	function hideSidebar() {
		showSidebar = false;
	}
</script>

<BaseNav
	containerClasses='container-fluid'
	topClasses="navbar-top border-bottom navbar-expand {type === 'default' ? 'bg-danger navbar-dark' : ''}"
	{type}
	menuClasses='navbar-collapse collapse'
	position='top'
	show={true}>
	<!-- Search form -->
	<form
		class="navbar-search form-inline mr-sm-3 {type === 'default' ? 'navbar-search-light' : ''}
    {type === 'light' ? 'navbar-search-dark' : ''}"
		id='navbar-search-main'>
		<fieldset class='form-group mb-0'>
			<div tabindex='-1' role='group' class='bv-no-focus-ring'>
				<div
					role='group'
					class='input-group input-group-alternative input-group-merge'>
					<div class='input-group-prepend'>
            <span class='input-group-text'>
              <i class='fas fa-search'></i>
            </span>
					</div>
					<input type='text' placeholder='Search' class='form-control' />
				</div>
			</div>
		</fieldset>
		<button
			type='button'
			class='close'
			data-action='search-close'
			data-target='#navbar-search-main'
			aria-label='Close'>
			<span aria-hidden='true'>×</span>
		</button>
	</form>
	<!-- Navbar links -->
	<ul class='navbar-nav align-items-center ml-md-auto'>
		<!-- This item dont have <b-nav-item> because they add class 'nav-link' which is not needed here -->
		<li class='nav-item d-xl-none'>
			<!-- Sidenav toggler -->
			<div
				class="pr-3 sidenav-toggler {type === 'default' ? 'sidenav-toggler-dark' : ''}
        {type === 'light' ? 'sidenav-toggler-light' : ''}
        {showSidebar === true ? 'active' : ''}"
				on:click={toggleSidebar}>
				<div class='sidenav-toggler-inner'>
					<i class='sidenav-toggler-line'></i>
					<i class='sidenav-toggler-line'></i>
					<i class='sidenav-toggler-line'></i>
				</div>
			</div>
		</li>
		<!-- This item dont have <b-nav-item> because item have data-action/data-target on tag <a>, wich we cant add -->
		<li class='nav-item d-sm-none'>
			<a
				class='nav-link'
				href='/'
				data-action='search-show'
				data-target='#navbar-search-main'>
				<i class='ni ni-zoom-split-in'></i>
			</a>
		</li>
		<BaseDropdown
			tagClasses='nav-item'
			tag='li'
			titleClasses='nav-link'
			titleTag='a'
			isOpen='false'
			icon='ni ni-bell-55'
			menuClasses='dropdown-menu-xl dropdown-menu-right py-0 overflow-hidden'>
			<!-- Dropdown header -->
			<div class='px-3 py-3'>
				<h6 class='text-sm text-muted m-0'>
					You have
					<strong class='text-primary'>13</strong>
					notifications.
				</h6>
			</div>
			<!-- List group -->
			<div class='list-group list-group-flush'>
				<div class='list-group-item' action href='#!'>
					<div class='row align-items-center'>
						<div class='col-md-auto'>
							<!-- Avatar -->
							<img
								src='/img/theme/team-1.jpg'
								alt='Image placeholder'
								class='avatar rounded-circle' />
						</div>
						<div class='col ml--2'>
							<div class='d-flex justify-content-between align-items-center'>
								<div>
									<h4 class='mb-0 text-sm'>John Snow</h4>
								</div>
								<div class='text-right text-muted'>
									<small>2 hrs ago</small>
								</div>
							</div>
							<p class='text-sm mb-0'>
								Let's meet at Starbucks at 11:30. Wdyt?
							</p>
						</div>
					</div>
				</div>
				<a href='#!' class='list-group-item list-group-item-action'>
					<div class='row align-items-center'>
						<div class='col-md-auto'>
							<!-- Avatar -->
							<img
								src='/img/theme/team-2.jpg'
								alt='Image placeholder'
								class='avatar rounded-circle' />
						</div>
						<div class='col ml--2'>
							<div class='d-flex justify-content-between align-items-center'>
								<div>
									<h4 class='mb-0 text-sm'>John Snow</h4>
								</div>
								<div class='text-right text-muted'>
									<small>3 hrs ago</small>
								</div>
							</div>
							<p class='text-sm mb-0'>
								A new issue has been reported for Argon.
							</p>
						</div>
					</div>
				</a>
				<div class='list-group-item' action href='#!'>
					<div class='row align-items-center'>
						<div class='col-md-auto'>
							<!-- Avatar -->
							<img
								src='/img/theme/team-3.jpg'
								alt='Image placeholder'
								class='avatar rounded-circle' />
						</div>
						<div class='col ml--2'>
							<div class='d-flex justify-content-between align-items-center'>
								<div>
									<h4 class='mb-0 text-sm'>John Snow</h4>
								</div>
								<div class='text-right text-muted'>
									<small>5 hrs ago</small>
								</div>
							</div>
							<p class='text-sm mb-0'>Your posts have been liked a lot.</p>
						</div>
					</div>
				</div>
				<div class='list-group-item' action href='#!'>
					<div class='row align-items-center'>
						<div class='col-md-auto'>
							<!-- Avatar -->
							<img
								src='/img/theme/team-4.jpg'
								alt='Image placeholder'
								class='avatar rounded-circle' />
						</div>
						<div class='col ml--2'>
							<div class='d-flex justify-content-between align-items-center'>
								<div>
									<h4 class='mb-0 text-sm'>John Snow</h4>
								</div>
								<div class='text-right text-muted'>
									<small>2 hrs ago</small>
								</div>
							</div>
							<p class='text-sm mb-0'>
								Let's meet at Starbucks at 11:30. Wdyt?
							</p>
						</div>
					</div>
				</div>
				<div class='list-group-item' action href='#!'>
					<div class='row align-items-center'>
						<div class='col-md-auto'>
							<!-- Avatar -->
							<img
								src='/img/theme/team-5.jpg'
								alt='Image placeholder'
								class='avatar rounded-circle' />
						</div>
						<div class='col ml--2'>
							<div class='d-flex justify-content-between align-items-center'>
								<div>
									<h4 class='mb-0 text-sm'>John Snow</h4>
								</div>
								<div class='text-right text-muted'>
									<small>3 hrs ago</small>
								</div>
							</div>
							<p class='text-sm mb-0'>
								A new issue has been reported for Argon.
							</p>
						</div>
					</div>
				</div>
			</div>
			<!-- View all -->
			<!-- This item dont have <b-dropdown-item> because item have styles " text-center text-primary font-weight-bold py-3" on tag <a> wich we cant add -->
			<a
				href='#!'
				class='dropdown-item text-center text-primary font-weight-bold py-3'>
				View all
			</a>
		</BaseDropdown>
		<BaseDropdown
			menuClasses='dropdown-menu-lg dropdown-menu-dark bg-default
      dropdown-menu-right'
			tagClasses='nav-item'
			tag='li'
			isOpen='false'
			titleTag='a'
			titleClasses='nav-link'
			icon='ni ni-ungroup'>
			<div class='row shortcuts px-4'>
				<a href='#!' class='col-4 shortcut-item'>
          <span class='shortcut-media avatar rounded-circle bg-gradient-red'>
            <i class='ni ni-calendar-grid-58'></i>
          </span>
					<small>Calendar</small>
				</a>
				<a href='#!' class='col-4 shortcut-item'>
          <span class='shortcut-media avatar rounded-circle bg-gradient-orange'>
            <i class='ni ni-email-83'></i>
          </span>
					<small>Email</small>
				</a>
				<a href='#!' class='col-4 shortcut-item'>
          <span class='shortcut-media avatar rounded-circle bg-gradient-info'>
            <i class='ni ni-credit-card'></i>
          </span>
					<small>Payments</small>
				</a>
				<a href='#!' class='col-4 shortcut-item'>
          <span class='shortcut-media avatar rounded-circle bg-gradient-green'>
            <i class='ni ni-books'></i>
          </span>
					<small>Reports</small>
				</a>
				<a href='#!' class='col-4 shortcut-item'>
          <span class='shortcut-media avatar rounded-circle bg-gradient-purple'>
            <i class='ni ni-pin-3'></i>
          </span>
					<small>Maps</small>
				</a>
				<a href='#!' class='col-4 shortcut-item'>
          <span class='shortcut-media avatar rounded-circle bg-gradient-yellow'>
            <i class='ni ni-basket'></i>
          </span>
					<small>Shop</small>
				</a>
			</div>
		</BaseDropdown>
	</ul>
	<ul class='align-items-center ml-auto ml-md-0 navbar-nav'>
		<BaseDropdown
			menuOnRight
			tagClasses='nav-item'
			tag='li'
			titleTag='a'
			titleClasses='nav-link pr-0'
			isOpen='false'>
			<a href='#!' class='nav-link pr-0' slot='title-container'>
				<div class='media align-items-center'>
          <span class='avatar avatar-sm rounded-circle'>
            <img alt='Image placeholder' src='..//img/theme/team-4.jpg' />
          </span>
					<div class='media-body ml-2 d-none d-lg-block'>
						<span class='mb-0 text-sm font-weight-bold'>John Snow</span>
					</div>
				</div>
			</a>

			<div class='dropdown-header noti-title'>
				<h6 class='text-overflow m-0'>Welcome!</h6>
			</div>
			<div class='dropdown-item' href='#!'>
				<i class='ni ni-single-02'></i>
				<span>My profile</span>
			</div>
			<div class='dropdown-item' href='#!'>
				<i class='ni ni-settings-gear-65'></i>
				<span>Settings</span>
			</div>
			<div class='dropdown-item' href='#!'>
				<i class='ni ni-calendar-grid-58'></i>
				<span>Activity</span>
			</div>
			<div class='dropdown-item' href='#!'>
				<i class='ni ni-support-16'></i>
				<span>Support</span>
			</div>
			<div class='dropdown-divider'></div>
			<div class='dropdown-item' href='#!'>
				<i class='ni ni-user-run'></i>
				<span>Logout</span>
			</div>

		</BaseDropdown>
	</ul>
</BaseNav>

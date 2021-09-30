<script>
	import users from '../../routes/users';
	import Card from '$components/Cards/Card.svelte';
	import SvelteTooltip from 'svelte-tooltip';

	function onEdit(row) {
		alert(`You want to edit ${row.name}`);
	}

	function onDelete(row) {
		alert(`You want to delete ${row.name}`);
	}

	import { onMount } from 'svelte';
	import jQuery from 'jquery';
	import dtCss from 'datatables.net-dt';

	let tableElement;
	onMount(() => {
			dtCss();

			// initialise table element
			jQuery(tableElement).DataTable({
				order: [[1, 'asc']],
				paging: false,
				searching: false,
				ordering: true,
				info: false,
				responsive: true
			});
		}
	);
</script>

<div>
	<Card noBody>
		{#if $$slots.tableTitle}
			<slot name='tableTitle'></slot>
		{/if}
		<div class='row w-100 mx-0 table-responsive'>
			<table
				bind:this={tableElement}
				class='table table-flush w-100 border-bottom-0'>
				<slot></slot>
			</table>
		</div>
	</Card>
</div>

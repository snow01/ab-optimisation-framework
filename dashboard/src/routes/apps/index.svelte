<script context='module'>
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		const url = `/apps.data`;
		const res = await fetch(url);

		if (res.ok) {
			let list = await res.json();
			console.log(list);

			return {
				props: {
					list
				}
			};
		}

		return {
			status: res.status,
			error: new Error(`Could not load ${url}`)
		};
	}
</script>

<script>
	import Page from '$layouts/Page.svelte';
	import Table from '$layouts/table/Table.svelte';
	import ButtonWithIcon from '$layouts/ButtonWithIcon.svelte';
	import TableHeader from '$layouts/table/TableHeader.svelte';
	import HeaderColumn from '$layouts/table/HeaderColumn.svelte';
	import Body from '$layouts/table/Body.svelte';
	import Row from '$layouts/table/Row.svelte';
	import Column from '$layouts/table/Column.svelte';

	export let list;

	let type = 'apps';
	let name = 'apps';
</script>

<Page {name} {type}>
	<div class='col-lg-6 col-5 text-right' slot='header'>
		<ButtonWithIcon icon='plus' title='New'></ButtonWithIcon>
	</div>
	<div class='container-fluid mt--5' slot='content'>
		<div class='row'>
			<div class='col' />
		</div>
		<Table>
<!--			<TableTitle slot='tableTitle' title='Applications'>-->
<!--				<ButtonWithIcon icon='user-edit' title='New'></ButtonWithIcon>-->
<!--			</TableTitle>-->
			<TableHeader>
				<HeaderColumn name='Name' sortable width=4></HeaderColumn>
				<HeaderColumn name='Short Name' sortable width=4></HeaderColumn>
				<HeaderColumn name='ID' sortable width=4></HeaderColumn>
			</TableHeader>
			<Body>
			{#each list as data, index}
				<Row id='1'>
					<Column width=4>{data.name}</Column>
					<Column width=4>{data.short_name}</Column>
					<Column width=4>{data.id}</Column>
				</Row>
			{/each}
			</Body>
		</Table>
	</div>
</Page>

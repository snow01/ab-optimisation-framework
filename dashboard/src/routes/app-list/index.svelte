<script context='module'>
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		const url = `/app-list.data`;
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
	export let list;

	let type = 'app-list';
	let name = 'apps';
</script>

<div class='container-fluid pt-3'>
	{#each list as json, index}
		<div class='row py-2'>
			{JSON.stringify(json)}
		</div>
	{/each}
</div>

<script context='module'>
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		const url = `/app-list.data`;
		const res = await fetch(url);

		if (res.ok) {
			return {
				props: {
					article: await res.json()
				}
			};
		}

		return {
			status: res.status,
			error: new Error(`Could not load ${url}`)
		};
	}
</script>


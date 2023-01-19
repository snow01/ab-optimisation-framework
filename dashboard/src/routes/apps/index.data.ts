/**
 * @type {import('@sveltejs/kit').RequestHandler}
 */
export async function get({ params }) {
	const url = `https://abof.myjosh.in/api/apps`;
	const res = await fetch(url);

	if (res.ok) {
		const data = await res.json()
		return {
			body: data
		};
	}

	return {
		status: res.status,
		body: new Error(`Could not load ${url}`)
	};
}
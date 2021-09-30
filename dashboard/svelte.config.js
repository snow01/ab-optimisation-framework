import preprocess from 'svelte-preprocess';
import path from 'path';
import node from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: [preprocess({})],

	kit: {
		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte',
		vite: () => ({
				resolve: {
					alias: {
						$assets: path.resolve('./src/assets'),
						$layouts: path.resolve('./src/layouts'),
						$components: path.resolve('./src/components')
					}
				}
		}),
		adapter: node()
	}
};

export default config;

import * as path from "path";

import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import license from "rollup-plugin-license";
import svgLoader from 'vite-svg-loader'

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [svelte(), svgLoader({
		defaultImport: 'raw',
		svgo: false
	})],
	resolve: {
		alias: [
			{ find: '@', replacement: path.resolve(__dirname, 'src') },
		],
	},
	build: {
		rollupOptions: {
			plugins: [
				license({
					thirdParty: {
						output: path.resolve(__dirname, "public/third-party-licenses.txt"),
					},
				}),
			],
		},
	},
	server: {
		port: 8080
	}
})


import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = ({
	plugins: [sveltekit()],
	server: {
		host: "127.0.0.1",
		port: 5173
	}
});

export default config;

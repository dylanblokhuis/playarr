import {load} from "dotenv";

export async function setupEnv() {
	const env = await load();

	Object.entries(env).forEach((item) => {
		const key = item[0];
		const value = item[1];

		console.log(`Loaded from .env: ${key}: ${value}`);
		Deno.env.set(key, value);
	});
}

import {getConfigValue} from "../api/config/repository.ts";

type methods = "GET" | "POST" | "PUT" | "DELETE";

export async function radarrApi(method: methods, path: string) {
	const address = await getConfigValue("radarr_address");
	const apiKey = await getConfigValue("radarr_api_key");

	return await (await fetch(`${address}/api/v3${path}`, {
		method: method,
		headers: {
			"X-Api-Key": apiKey
		}
	})).json();
}

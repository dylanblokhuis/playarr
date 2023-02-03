import {getConfigValue} from "../api/config/repository.ts";

type methods = "GET" | "POST" | "PUT" | "DELETE";

export async function sonarrApi(method: methods, path: string) {
	const address = await getConfigValue("sonarr_address");
	const apiKey = await getConfigValue("sonarr_api_key");

	return await (await fetch(`${address}/api/v3${path}`, {
		method: method,
		headers: {
			"X-Api-Key": apiKey
		}
	})).json();
}

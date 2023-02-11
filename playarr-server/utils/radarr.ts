type methods = "GET" | "POST" | "PUT" | "DELETE";

export async function radarrApi(method: methods, path: string) {
	const address = Deno.env.get("RADARR_ADDRESS")
	const apiKey = Deno.env.get("RADARR_API_KEY")

	if (!address || !apiKey) {
		throw new Error("Missing Radarr address or API key");
	}

	return await (await fetch(`${address}/api/v3${path}`, {
		method: method,
		headers: {
			"X-Api-Key": apiKey
		}
	})).json();
}

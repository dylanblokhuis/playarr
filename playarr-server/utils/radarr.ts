type methods = "GET" | "POST" | "PUT" | "DELETE";

export async function radarrApi(method: methods, path: string) {
	const address = Deno.env.get("RADARR_ADDRESS");
	const apiKey = Deno.env.get("RADARR_API_KEY");

	if (!address || !apiKey) throw new Error("Radarr address or API key not set");

	return await (await fetch(`${address}/api/v3${path}`, {
		method: method,
		headers: {
			"X-Api-Key": apiKey
		}
	})).json();
}

type methods = "GET" | "POST" | "PUT" | "DELETE";

export async function sonarrApi(method: methods, path: string) {
	const address = Deno.env.get("SONARR_ADDRESS")
	const apiKey = Deno.env.get("SONARR_API_KEY");

	if (!address || !apiKey) throw new Error("Sonarr address or API key not set");

	return await (await fetch(`${address}/api/v3${path}`, {
		method: method,
		headers: {
			"X-Api-Key": apiKey
		}
	})).json();
}

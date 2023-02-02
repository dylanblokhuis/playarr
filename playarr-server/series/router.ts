/* This file has been generated with "deno task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import db from "../db/instance.ts";

const seriesRouter = new Router();
seriesRouter
	.get("/series", async ({response}) => {
		// Todo: Add these to the config repo with getConfig(name: string)
		const sonarr_address = (await db.selectFrom("config")
			.selectAll()
			.where("name", "=", "sonarr_address")
			.executeTakeFirst())?.value;

		// Todo: there must be a better way for this
		if (!sonarr_address) {
			response.status = 400;
			response.body = "Please set sonarr_address in the settings";
			return;
		}

		const sonarr_api_key = (await db.selectFrom("config")
			.selectAll()
			.where("name", "=", "sonarr_api_key")
			.executeTakeFirst())?.value;

		if (!sonarr_api_key) {
			response.status = 400;
			response.body = "Please set sonarr_api_key in the settings";
			return;
		}

		//todo:  Create add some kind of "getSonarrUrl(path: string)" method, keeps things clean, or maybe create a simple http client
		const series = await (await fetch(`${sonarr_address}/api/v3/series`, {
			method: "GET",
			headers: {
				"X-Api-Key": sonarr_api_key
			}
		})).json();

		response.body = series;
	})
	.get("/series/:series_id/episodes", async ({params, response}) => {
		const sonarr_address = (await db.selectFrom("config")
			.selectAll()
			.where("name", "=", "sonarr_address")
			.executeTakeFirst())?.value;

		// Todo: there must be a better way for this
		if (!sonarr_address) {
			response.status = 400;
			response.body = "Please set sonarr_address in the settings";
			return;
		}

		const sonarr_api_key = (await db.selectFrom("config")
			.selectAll()
			.where("name", "=", "sonarr_api_key")
			.executeTakeFirst())?.value;

		if (!sonarr_api_key) {
			response.status = 400;
			response.body = "Please set sonarr_api_key in the settings";
			return;
		}

		let episodes = await (await fetch(`${sonarr_address}/api/v3/episode?seriesId=${params.series_id}&includeImages=true`, {
			method: "GET",
			headers: {
				"X-Api-Key": sonarr_api_key
			}
		})).json();

		// Only show episodes that are supposed to be there
		episodes = episodes.filter(e => e.monitored);

		response.body = episodes;
	})
	.get("/episode/:episode_id", async ({params, response}) => {
	const sonarr_address = (await db.selectFrom("config")
		.selectAll()
		.where("name", "=", "sonarr_address")
		.executeTakeFirst())?.value;

	// Todo: there must be a better way for this
	if (!sonarr_address) {
		response.status = 400;
		response.body = "Please set sonarr_address in the settings";
		return;
	}

	const sonarr_api_key = (await db.selectFrom("config")
		.selectAll()
		.where("name", "=", "sonarr_api_key")
		.executeTakeFirst())?.value;

	if (!sonarr_api_key) {
		response.status = 400;
		response.body = "Please set sonarr_api_key in the settings";
		return;
	}

	const episode = await (await fetch(`${sonarr_address}/api/v3/episode/${params.episode_id}`, {
		method: "GET",
		headers: {
			"X-Api-Key": sonarr_api_key
		}
	})).json();

	response.body = episode;
});

export default seriesRouter;

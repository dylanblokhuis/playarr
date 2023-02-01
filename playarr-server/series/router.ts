/* This file has been generated with "dano task create_package" */
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

		//todo:  Create add some kind of "getSonarrUrl(path: string)" method, keeps things clean
		const series = await (await fetch(`${sonarr_address}/api/v3/series`, {
			method: "GET",
			headers: {
				"X-Api-Key": sonarr_api_key
			}
		})).json()

		response.body = series;
	});

export default seriesRouter;

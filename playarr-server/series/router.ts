/* This file has been generated with "dano task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import db from "../db/instance.ts";

const seriesRouter = new Router();
seriesRouter
	.get("/series", async ({response}) => {
		const sonarr_address = (await db.selectFrom("config")
			.selectAll()
			.where("name", "=", "sonarr_address")
			.executeTakeFirst())?.value;

		console.log(sonarr_address)

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

		const series = await (await fetch(`${sonarr_address}/api/v3/series`, {
			method: "GET",
			headers: {
				"X-Api-Key": sonarr_api_key
			}
		})).json()

		console.log(series);


		response.body = series;
	});

export default seriesRouter;

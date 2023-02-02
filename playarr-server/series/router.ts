/* This file has been generated with "deno task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import {sonarrApi} from "../external/sonarr.ts";

const seriesRouter = new Router();

seriesRouter
	.get("/series", async ({response}) => {
		try {
			response.body = await sonarrApi("GET", "/series");
		} catch (e) {
			response.status = 400;
			response.body = e.message;
		}
	})
	.get("/series/:series_id/episodes", async ({params, response}) => {
		try {
			let episodes = await sonarrApi("GET", `/episode?seriesId=${params.series_id}&includeImages=true`);

			// Only show episodes that are supposed to be there
			episodes = episodes.filter(e => e.monitored);
			response.body = episodes;
		} catch (e) {
			response.status = 400;
			response.body = e.message;
		}
	})
	.get("/episode/:episode_id", async ({params, response}) => {
		try {
			response.body = await sonarrApi("GET", `/episode/${params.episode_id}`);
		} catch (e) {
			response.status = 400;
			response.body = e.message;
		}
	});

export default seriesRouter;

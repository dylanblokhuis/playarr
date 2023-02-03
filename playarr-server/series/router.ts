/* This file has been generated with "deno task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import {sonarrApi} from "../utils/sonarr.ts";

const seriesRouter = new Router();

seriesRouter
	.get("/series", async ({response, throw: error}) => {
		try {
			response.body = await sonarrApi("GET", "/series");
		} catch (e) {
			error(400, e.message);
		}
	})
	.get("/series/:series_id/episodes", async ({params, response, throw: error}) => {
		try {
			const episodes = await sonarrApi("GET", `/episode?seriesId=${params.series_id}&includeImages=true`);

			// Only show episodes that are supposed to be there
			response.body = episodes.filter(e => e.monitored);
		} catch (e) {
			error(400, e.message);
		}
	})
	.get("/episode/:episode_id", async ({params, response, throw: error}) => {
		try {
			response.body = await sonarrApi("GET", `/episode/${params.episode_id}`);
		} catch (e) {
			error(400, e.message);
		}
	});

export default seriesRouter;

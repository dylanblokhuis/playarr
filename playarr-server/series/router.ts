/* This file has been generated with "deno task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import {sonarrApi} from "./repository.ts";

const seriesRouter = new Router();

seriesRouter
	.get("/series", async ({response}) => {
		response.body = await sonarrApi("GET", "/series");
	})
	.get("/series/:series_id/episodes", async ({params, response}) => {
		let episodes = await sonarrApi("GET", `/episode?seriesId=${params.series_id}&includeImages=true`);

		// Only show episodes that are supposed to be there
		episodes = episodes.filter(e => e.monitored);

		response.body = episodes;
	})
	.get("/episode/:episode_id", async ({params, response}) => {
		response.body = await sonarrApi("GET", `/episode/${params.episode_id}`);
	});

export default seriesRouter;

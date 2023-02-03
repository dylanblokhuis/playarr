/* This file has been generated with "deno task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import {getEpisodeById, getEpisodesByShow, getShows} from "./repository.ts";

const tvRouter = new Router();

tvRouter
	.get("/tv", async ({response, throw: error}) => {
		try {
			response.body = await getShows();
		} catch (e) {
			error(400, e.message);
		}
	})
	.get("/tv/:show_id/episodes", async ({params, response, throw: error}) => {
		try {
			const episodes = await getEpisodesByShow(params.show_id);

			// Only show episodes that are supposed to be there
			response.body = episodes.filter((e: any) => e.monitored);
		} catch (e) {
			error(400, e.message);
		}
	})
	.get("/episode/:episode_id", async ({params, response, throw: error}) => {
		try {
			response.body = await getEpisodeById(params.episode_id);
		} catch (e) {
			error(400, e.message);
		}
	});

export default tvRouter;

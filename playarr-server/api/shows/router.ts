/* This file has been generated with "deno task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import {getEpisodeById, getEpisodesByShow, getShows} from "./repository.ts";

const showsRouter = new Router();
showsRouter
	.get("/shows", async ({response, throw: error}) => {
		try {
			response.body = await getShows();
		} catch (e) {
			await error(e.status, e.message);
		}
	})
	.get("/shows/:show_id/episodes", async ({params, response, throw: error}) => {
		try {
			const episodes = await getEpisodesByShow(params.show_id);

			// Only show episodes that are supposed to be there
			response.body = episodes.filter((e: any) => e.monitored);
		} catch (e) {
			await error(e.status, e.message);
		}
	})
	.get("/episodes/:episode_id", async ({params, response, throw: error}) => {
		try {
			response.body = await getEpisodeById(params.episode_id);
		} catch (e) {
			await error(e.status, e.message);
		}
	});

export default showsRouter;

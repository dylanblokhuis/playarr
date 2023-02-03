/* This file has been generated with "deno task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import {getMovieById, getMovies} from "./repository.ts";

const moviesRouter = new Router();
moviesRouter
	.get("/movies", async ({response, throw: error}) => {
		try {
			response.body = await getMovies();
		} catch (e) {
			await error(e.status, e.message);
		}
	})
	.get("/movies/:movie_id", async ({params, response, throw: error}) => {
		try {
			// Maybe this is not needed since getMovies() already returns everything
			response.body = await getMovieById(params.movie_id);
		} catch (e) {
			await error(e.status, e.message);
		}
	})

export default moviesRouter;

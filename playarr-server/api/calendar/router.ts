/* This file has been generated with "deno task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import {getRadarrCalendar, getSonarrCalendar} from "./repository.ts";

const calendarRouter = new Router();
calendarRouter
	.get("/calendar", async ({response}) => {
		const shows = await getSonarrCalendar();
		const movies = await getRadarrCalendar();

		response.body = {shows, movies};
	});

export default calendarRouter;

import { Application } from "https://deno.land/x/oak@v11.1.0/application.ts";
// import configRouter from "./../api/config/router.ts";
import showsRouter from "./../api/shows/router.ts";
import moviesRouter from "./../api/movies/router.ts";
import calendarRouter from "./../api/calendar/router.ts";

const routers = [
	// configRouter,
	showsRouter,
	moviesRouter,
	calendarRouter
];

export function setupRouters(app: Application) {
	for (const router of routers) {
		app.use(router.routes());
		app.use(router.allowedMethods());
	}
}

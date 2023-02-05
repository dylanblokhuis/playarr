import { Application } from "oak";
import configRouter from "./api/config/router.ts";
import showsRouter from "./api/shows/router.ts";
import moviesRouter from "./api/movies/router.ts";
import calendarRouter from "./api/calendar/router.ts";

const app = new Application();

// Catch all errors thrown by error()
app.use(async (context, next) => {
	try {
		await next();
	} catch (e) {
		context.response.status = e.status;
		context.response.body = e.message;
	}
});

const routers = [
	configRouter,
	showsRouter,
	moviesRouter,
	calendarRouter
];

for (const router of routers) {
	app.use(router.routes());
	app.use(router.allowedMethods());
}

const port = 8000
console.log(`listening on http://localhost:${port}`)
await app.listen({ port });

import {Application} from "oak";
import configRouter from "./api/config/router.ts";
import showsRouter from "./api/shows/router.ts";

const app = new Application();

// Catch all errors thrown by error()
app.use(async (context, next) => {
	try {
		await next();
	} catch (e) {
		context.response.body = e.message;
	}
});

const routers = [
	configRouter,
	showsRouter
];

for (const router of routers) {
	app.use(router.routes());
	app.use(router.allowedMethods());
}

await app.listen({port: 8000});

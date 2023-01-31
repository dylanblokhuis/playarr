import {Application} from "oak";
import configRouter from "config/router.ts";

const app = new Application();
const routers = [
	configRouter
]

for (const router in routers) {
	app.use(router.routes());
	app.use(router.allowedMethods());
}

await app.listen({port: 8000});

import {Application} from "oak";
import configRouter from "./config/router.ts";

const app = new Application();
const routers = [
	configRouter
];

for (const router of routers) {
	app.use(router.routes());
	app.use(router.allowedMethods());
}

await app.listen({port: 8000});

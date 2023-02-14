import {Application} from "oak";
import {setupRouters} from "./startup/routers.ts";
import {setupMiddleware} from "./startup/middleware.ts";
import {setupEnv} from "./startup/env.ts";

await setupEnv();

const app = new Application();
setupMiddleware(app);
setupRouters(app);

const port = 8000;
console.log(`Listening on port ${port}`);
await app.listen({port, hostname: "0.0.0.0"});

import { Application } from "oak";
import { setupRouters } from "./startup/routers.ts";
import { setupMiddleware } from "./startup/middleware.ts";
import { migrate } from "./tasks/migrate.ts";

const app = new Application();
setupMiddleware(app);
setupRouters(app);

migrate();

const port = 8000;
console.log(`Listening on port ${port}`);
await app.listen({ port, hostname: "0.0.0.0" });

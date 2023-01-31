import {Application, Router, send} from "oak";

const router = new Router();
router
	.get("/", ({response}) => {
		response.body = "Go to /73 for vod";
	})
	.get("/73", (context) => {
		return send(context, "./vods/73.mp4");
	});


const app = new Application();
app.use(router.routes());
app.use(router.allowedMethods());

await app.listen({port: 8000});

import {Application, Router, send} from "oak";
import db, {migrate} from "./db/instance.ts";

// migrate().then(() => {
// 	db
// 		.insertInto("user")
// 		.values({name: "jari", email: "jari@kruitbos.dev", password: "pog", role: "admin"})
// 		.returningAll()
// 		.execute()
// 		.then((result) => {
// 			console.log(result)
// 		})
// })

const router = new Router();
router
	.get("/", ({response}) => {
		response.body = "Go to /73 for vod"
	})
	.get("/73", (context) => {
		return send(context, "./vods/73.mp4");
	});


const app = new Application();
app.use(router.routes());
app.use(router.allowedMethods());

await app.listen({port: 8000});

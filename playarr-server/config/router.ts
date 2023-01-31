import {Router} from "oak";

const router = new Router();
router
	.get("/", ({response}) => {
		response.body = "Hello world!";
	})

export default router as configRouter

import {Router} from "oak";

const configRouter = new Router();
configRouter
	.get("/", ({response}) => {
		response.body = "Hello world!";
	});

export default configRouter;

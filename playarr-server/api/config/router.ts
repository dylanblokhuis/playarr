import {Router} from "oak";
import {getAllConfigs, getConfigByName, setConfigValue} from "./repository.ts";

const configRouter = new Router();
configRouter
	.get("/config", async ({response}) => {
		response.body = await getAllConfigs();
	})
	.get("/config/:name", async ({params, response, throw: error}) => {
		try {
			response.body = await getConfigByName(params.name);
		} catch (e) {
			await error(404, e.message);
		}
	})
	.put("/config/:name", async ({request, params, response, throw: error}) => {
		const value = await request.body().value;
		const configValue = value?.value;

		if (!configValue) await error(400, "No value given");

		try {
			response.body = await setConfigValue(params.name, configValue);
		} catch (e) {
			await error(404, e.message);
		}

	});

export default configRouter;

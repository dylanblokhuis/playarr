import {Router} from "oak";
import db from "../db/instance.ts";

const configRouter = new Router();
configRouter
	.get("/config", async ({response}) => {
		response.body = await db.selectFrom("config")
			.selectAll()
			.execute();
	})
	.get("/config/:name", async ({response, params}) => {
		response.body = await db.selectFrom("config")
			.selectAll()
			.where("name", "=", params.name)
			.executeTakeFirst();
	})
	.put("/config/:name", async ({request, params, response}) => {
		// Todo: refactor on how to get request values, add some repository type abstraction layer for db interaction
		const value = await request.body().value;
		const configValue = value.value ?? null;

		if (!configValue) {
			response.status = 400;
			response.body = "No value given"
			return;
		}

		response.body = await db.updateTable("config")
			.set({value: configValue})
			.where("name", "=", params.name)
			.returningAll()
			.executeTakeFirst();
	})

export default configRouter;

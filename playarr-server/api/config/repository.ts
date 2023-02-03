import db from "../../db/instance.ts";
import {ConfigNotFoundException} from "./execptions.ts";
import {ConfigRow} from "./db/tables.ts";

export async function getAllConfigs(): Promise<ConfigRow[]>  {
	return await db.selectFrom("config")
		.selectAll()
		.execute();
}

export async function getConfigByName(name: string): Promise<ConfigRow> {
	const config = await db.selectFrom("config")
		.selectAll()
		.where("name", "=", name)
		.executeTakeFirst();

	if (!config) throw new ConfigNotFoundException(name);
	return config;
}

export async function setConfigValue(name: string, value: string): Promise<ConfigRow> {
	const config = await db.updateTable("config")
		.set({value: value})
		.where("name", "=", name)
		.returningAll()
		.executeTakeFirst();

	if (!config) throw new ConfigNotFoundException(name);
	return config;
}

export async function getConfigValue(name: string): Promise<string> {
	const config = await getConfigByName(name);
	return config.value ?? "";
}

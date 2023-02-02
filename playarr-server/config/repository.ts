import db from "../db/instance.ts";
import {ConfigNotFoundException} from "./execptions.ts";

export async function getConfigValue(name: string): Promise<string> {
	const value = (await db.selectFrom("config")
		.selectAll()
		.where("name", "=", name)
		.where("value", "!=", "\"\"")
		.executeTakeFirst())?.value;

	if (!value) {
		throw new ConfigNotFoundException(name);
	}

	return value;
}

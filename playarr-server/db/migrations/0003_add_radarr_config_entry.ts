/* This file has been generated with "deno task create_migration" */

import { Kysely } from "kysely";
import { Database } from "../instance.ts";

export async function up(db: Kysely<Database>): Promise<void> {
	await db.insertInto("config")
		.values({name: "radarr_address", value: ""})
		.execute();

	await db.insertInto("config")
		.values({name: "radarr_api_key", value: ""})
		.execute();
}

export async function down(db: Kysely<Database>): Promise<void> {
	await db.deleteFrom("config")
		.where("config.name", "=", "radarr_api_key")
		.execute();

	await db.deleteFrom("config")
		.where("config.name", "=", "radarr_address")
		.execute();
}

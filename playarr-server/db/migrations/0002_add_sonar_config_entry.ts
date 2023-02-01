/* This file has been generated with "dano task create_migration" */

import { Kysely } from "kysely";
import { Database } from "../instance.ts";

export async function up(db: Kysely<Database>): Promise<void> {
	await db.insertInto("config")
		.values({name: "sonarr_address", value: ""})
		.execute();

	await db.insertInto("config")
		.values({name: "sonarr_api_key", value: ""})
		.execute();
}

export async function down(db: Kysely<Database>): Promise<void> {
	await db.deleteFrom("config")
		.where("config.name", "=", "sonarr_api_key")
		.execute();

	await db.deleteFrom("config")
		.where("config.name", "=", "sonarr_address")
		.execute();
}

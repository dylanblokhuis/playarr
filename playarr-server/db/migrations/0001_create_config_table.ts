/* This file has been generated with "dano task create_migration" */

import { Kysely, sql } from "kysely";
import { Database } from "../instance.ts";

export async function up(db: Kysely<Database>): Promise<void> {
	await db.schema
		.createTable("config")
		.addColumn("id", "integer", (col) => col.primaryKey().autoIncrement())
		.addColumn("name", "varchar", (col) => col.notNull().unique())
		.addColumn("value", "varchar", (col) => col.notNull())
		.addColumn(
			"updated_at",
			"datetime",
			(col) => col.notNull().defaultTo(sql`current_timestamp`),
		)
		.execute();
}

export async function down(db: Kysely<unknown>): Promise<void> {
	await db.schema.dropTable("config").execute();
}

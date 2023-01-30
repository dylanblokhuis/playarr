import { Kysely, sql } from "kysely";
import { Database } from "../instance.ts";

export async function up(db: Kysely<Database>): Promise<void> {
  await db.schema
    .createTable("user")
    .addColumn("id", "integer", (col) => col.primaryKey().autoIncrement())
    .addColumn("name", "varchar", (col) => col.notNull())
    .addColumn("email", "varchar", (col) => col.notNull())
    .addColumn("password", "varchar", (col) => col.notNull())
    .addColumn("role", "varchar", (col) => col.notNull())
    .addColumn(
      "created_at",
      "datetime",
      (col) => col.notNull().defaultTo(sql`current_timestamp`),
    )
    .addColumn(
      "updated_at",
      "datetime",
      (col) => col.notNull().defaultTo(sql`current_timestamp`),
    )
    .execute();
}

export async function down(db: Kysely<unknown>): Promise<void> {
  await db.schema.dropTable("user").execute();
}

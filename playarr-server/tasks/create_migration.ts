const template = `/* This file has been generated with "deno task create_migration" */

import { Kysely, sql } from "kysely";
import { Database } from "../instance.ts";

export async function up(db: Kysely<Database>): Promise<void> {
	// ...
}

export async function down(db: Kysely<Database>): Promise<void> {
	// ...
}
`;

/**
 * Return a name like 0006_migration_name or 0174_name_migration
 * @param name
 * @param version
 */
function getMigrationName(name: string, version: number): string {
	// Turn the string into snake_case
	const snake_case = name.replace(/\W+/g, " ")
		.split(/ |\B(?=[A-Z])/)
		.map(word => word.toLowerCase())
		.join('_');

	let v = version.toString()
	v = v.padStart(4, "0");

	return `${v}_${snake_case}.ts`;
}

async function createMigration(migrationName: string): Promise<string> {
	const root = Deno.cwd();
	const migrationsPath = `${root}/db/migrations`;

	let version = 1;
	for await (const _migration of Deno.readDir(migrationsPath)) {
		version++;
	}

	const name = getMigrationName(migrationName, version);
	const migration = `${migrationsPath}/${name}`

	await Deno.writeTextFile(migration, template);
	return migration;
}


if (Deno.args.length < 1) {
	console.log("Usage: deno task create_migration <migration_name>");
	Deno.exit(1);
}

const migrationName = Deno.args[0];

createMigration(migrationName).then((migration) => {
	console.log(`Created migration for successfully! ${migration}`);
}).catch((error) => {
	console.error(`Could not create migration: ${error}`);
});

// Some blacklisted package names, to make sure we are not getting any migrations here
const packagesBlacklist = ["db"];

const template = `/* This file has been generated with "dano task create_migration" */

import { Kysely, sql } from "kysely";
import { Database } from "../../../db/instance.ts";

export async function up(db: Kysely<Database>): Promise<void> {
	// ...
}

export async function down(db: Kysely<unknown>): Promise<void> {
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

async function createMigration(packageName: string, migrationName: string): Promise<string> {
	const root = Deno.cwd();
	const path = `${root}/${packageName}`;

	// Check if the package exists
	const migration = await Deno.stat(path).then(async () => {
		// Create a migrations' folder, will succeed silently if already exists thanks to "recursive"
		const migrationsPath = `${path}/db/migrations`;
		await Deno.mkdir(migrationsPath, {recursive: true});

		let version = 1;
		for await (const _migration of Deno.readDir(migrationsPath)) {
			version++;
		}

		const name = getMigrationName(migrationName, version);
		return `${migrationsPath}/${name}`
	}).catch(() => {
		throw new Error(`Could not find package '${packageName}'`);
	});

	await Deno.writeTextFile(migration, template);
	return migration;
}


if (Deno.args.length < 2) {
	console.log("Usage: deno task create_migration <package> <migration_name>");
	Deno.exit(1);
}

const packageName = Deno.args[0];
const migrationName = Deno.args[1];

if (packagesBlacklist.includes(migrationName)) {
	console.error(`Could not create migration for ${packageName}: package has been blacklisted`);
	Deno.exit(1);
}

createMigration(packageName, migrationName).then((migration) => {
	console.log(`Created migration for ${packageName} successfully! ${migration}`);
}).catch((error) => {
	console.error(`Could not create migration for ${packageName}: ${error}`);
});

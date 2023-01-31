const template = `import { Kysely, sql } from "kysely";
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

async function createMigration(app: string, name: string): Promise<{name: string, path: string}> {
	const root = Deno.cwd();
	const path = `${root}/${app}`;

	// Check if the folder exists
	const migration = await Deno.stat(path).then(async () => {
		// Create a migrations' folder, will succeed silently if already exists thanks to "recursive"
		const migrationsPath = `${path}/db/migrations`;
		await Deno.mkdir(migrationsPath, {recursive: true});

		let version = 1;
		for await (const _migration of Deno.readDir(migrationsPath)) {
			version++;
		}

		const migrationName = getMigrationName(name, version);
		console.log(version, migrationName);

		const fullPath = `${migrationsPath}/${migrationName}`;
		return {
			name: migrationName,
			path: fullPath
		}

	}).catch(() => {
		throw new Error(`Could not find app '${app}'`);
	});

	await Deno.writeTextFile(migration.path, template);
	return migration;
}


if (Deno.args.length < 2) {
	console.log("Usage: deno task create_migration <app> <migration_name>");
	Deno.exit(1);
}

const app = Deno.args[0];
const name = Deno.args[1];

createMigration(app, name).then((migration) => {
	console.log(`Created migration for ${app} successfully! ${migration.name} [${migration.path}]`);
}).catch((error) => {
	console.error(`Could not create migration for ${app}: ${error}`);
});

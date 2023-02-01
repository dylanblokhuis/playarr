import {Migration, Migrator} from "kysely";
import * as path from "std/path/mod.ts";

import db from "../db/instance.ts";

async function migrate() {
	const migrator = new Migrator({
		db: db,
		provider: {
			getMigrations: async () => {
				const migrations: Record<string, Migration> = {};
				const root = Deno.cwd();

				const migrationsDirectory = `${root}/db/migrations`;
				const migrationFiles = await Deno.readDir(migrationsDirectory);

				for await (const migration of migrationFiles) {
					const {up, down} = await import(
						path.join(migrationsDirectory, `./${migration.name}`)
						);

					migrations[migration.name] = {
						up,
						down,
					};
				}

				return migrations;
			},
		},
	});

	const {error, results} = await migrator.migrateToLatest();
	if (results && results.length > 0) {
		console.log(results);
	}

	if (error) {
		const migrationError = error as Error;
		throw migrationError.message;
	}
}

migrate().then(() => {
	console.log("Finished migrating!");
}).catch((e) => {
	console.error(`Errors while migrating: ${e}`);
});

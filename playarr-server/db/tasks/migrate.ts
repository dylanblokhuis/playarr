import {Migration, Migrator} from "kysely";
import * as path from "std/path/mod.ts";

import db from "../instance.ts";

async function migrate() {
	const migrator = new Migrator({
		db: db,
		provider: {
			getMigrations: async () => {
				const migrations: Record<string, Migration> = {};
				const root = Deno.cwd();

				// First we get all the folders in the root directory
				for await (const fileOrDirectory of Deno.readDir(root)) {
					if (fileOrDirectory.isDirectory) {

						// Check if this folder got a db/migrations folder
						const migrationsDirectory = `${root}/${fileOrDirectory.name}/db/migrations`;
						await Deno.stat(migrationsDirectory).then(async () => {

							// If so, lets grab the migrations
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
						}).catch(() => {
							// No migrations found, but let's make the console happy
						});
					}
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

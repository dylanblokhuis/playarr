import {Kysely, Migration, Migrator, SqliteAdapter, SqliteIntrospector, SqliteQueryCompiler} from "kysely";
import {Database as SqliteDatabase} from "sqlite";
import {SqliteDriver} from "./driver.ts";
import * as path from "std/path/mod.ts";

import {ConfigTable} from "../config/db/tables.ts";

const sqlite = new SqliteDatabase("playarr.db");

export interface Database {
	config: ConfigTable;
}

const db = new Kysely<Database>({
	dialect: {
		createAdapter() {
			return new SqliteAdapter();
		},
		createDriver() {
			return new SqliteDriver({
				database: sqlite,
			});
		},
		createIntrospector(db: Kysely<unknown>) {
			return new SqliteIntrospector(db);
		},
		createQueryCompiler() {
			return new SqliteQueryCompiler();
		},
	},
});


export async function migrate() {
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
						})
					}
				}
				console.log(migrations)
				console.log("return")
				return migrations;
				// const migrationDirPath = path.join(
				// 	Deno.cwd(),
				// 	"./db/migrations",
				// );
				//
				// const migrationFiles = await Deno.readDir(migrationDirPath);
				// const migrations: Record<string, Migration> = {};
				//
				// for await (const migration of migrationFiles) {
				// 	const {up, down} = await import(
				// 		path.join(migrationDirPath, `./${migration.name}`)
				// 	);
				//
				// 	migrations[migration.name] = {
				// 		up,
				// 		down,
				// 	};
				// }
				//
				// return migrations;
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

export default db;

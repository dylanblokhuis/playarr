import {
	ColumnType,
	Generated,
	Kysely,
	Migration,
	Migrator,
	SqliteAdapter,
	SqliteIntrospector,
	SqliteQueryCompiler,
} from "kysely";
import {Database as SqliteDatabase} from "sqlite";
import {SqliteDriver} from "./driver.ts";
import * as path from "std/path/mod.ts";

const sqlite = new SqliteDatabase("playarr.db");

export type Roles = "admin" | "regular";

interface UserTable {
	id: Generated<number>;
	name: string;
	email: string;
	password: string;
	role: Roles;
	created_at: ColumnType<Date, string | undefined, never>;
	updated_at: ColumnType<Date, string | undefined, never>;
}

export interface Database {
	user: UserTable;
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
				const migrationDirPath = path.join(
					Deno.cwd(),
					"./db/migrations",
				);
				const migrationFiles = await Deno.readDir(migrationDirPath);
				const migrations: Record<string, Migration> = {};
				for await (const migration of migrationFiles) {
					const {up, down} = await import(
						path.join(migrationDirPath, `./${migration.name}`)
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

export default db;

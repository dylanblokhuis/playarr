import {Kysely, SqliteAdapter, SqliteIntrospector, SqliteQueryCompiler} from "kysely";
import {Database as SqliteDatabase} from "sqlite";
import {SqliteDriver} from "./driver.ts";


import {ConfigTable} from "../api/config/db/tables.ts";

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

export default db;

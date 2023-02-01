import {Generated, ColumnType} from "kysely";

export interface ConfigTable {
	id: Generated<number>;
	name: string;
	value: string;
	updated_at: ColumnType<Date, string  | undefined, never>;
}

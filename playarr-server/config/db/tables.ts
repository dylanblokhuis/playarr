import {Generated, ColumnType, Selectable, Updateable, Insertable} from "kysely";

export interface ConfigTable {
	id: Generated<number>;
	name: string;
	value: string;
	updated_at: ColumnType<Date, string  | undefined, never>;
}


export type ConfigRow = Selectable<ConfigTable> |
	Insertable<ConfigTable> |
	Updateable<ConfigTable>

import { Generated } from "kysely";

export interface ConfigTable {
	id: Generated<number>;
	name: string;
	value: string;
}

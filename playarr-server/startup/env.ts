import {load} from "dotenv";

export async function setupEnv() {
	await load({allowEmptyValues: true, export: true});
}

const tablesTemplate = `/* This file has been generated with "dano task create_package" */
/* Do not forget to add me to the Database type */

import {Generated, ColumnType} from "kysely";

export interface SomeTable {
	// ...
}
`;

const routerTemplate = `/* This file has been generated with "dano task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";

const router = new Router();
router
	.get("/", ({response}) => {
		response.body = "Hello world!";
	})

export default router as someRouter
`;

async function createPackage(name: string) {
	const root = Deno.cwd();
	const path = `${root}/${name}`;

	const routerFile = `${path}/router.ts`;
	const tablesFile = `${path}/db/tables.ts`;

	await Deno.mkdir(path);
	await Deno.mkdir(`${path}/db`);
	await Deno.writeTextFile(routerFile, routerTemplate);
	await Deno.writeTextFile(tablesFile, tablesTemplate);

	// Todo: Do I need to close this?
	// Create the first migration
	Deno.run({
		cmd: ["deno", "task", "create_migration", name, "setup_and_seed"]
	});
}


if (Deno.args.length < 1) {
	console.log("Usage: deno task create_package <package_name>");
	Deno.exit(1);
}

const name = Deno.args[0];

createPackage(name).then(() => {
	console.log("Package created successfully");
}).catch((error) => {
	console.error(`Could not create package: ${error}`);
});



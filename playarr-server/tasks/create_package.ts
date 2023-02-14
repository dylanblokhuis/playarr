const repositoryTemplate = `/* This file has been generated with "deno task create_package" */

export async function someFunction() {
	return "hello world";
}
`;


const routerTemplate = `/* This file has been generated with "deno task create_package" */
/* Do not forget to add me to the routers in app.ts */

import {Router} from "oak";
import {someFunction} from "./repository.ts";

const someRouter = new Router();
someRouter
	.get("/", async ({response}) => {
		response.body = await someFunction();
	});

export default someRouter;
`;

async function createPackage(name: string) {
	const root = Deno.cwd();
	const path = `${root}/api/${name}`;

	const routerFile = `${path}/router.ts`;
	const repositoryFile = `${path}/repository.ts`;

	await Deno.mkdir(path);
	await Deno.writeTextFile(routerFile, routerTemplate);
	await Deno.writeTextFile(repositoryFile, repositoryTemplate);
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



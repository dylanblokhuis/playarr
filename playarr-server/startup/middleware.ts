import {Application} from "https://deno.land/x/oak@v11.1.0/application.ts";

// deno-lint-ignore no-explicit-any
async function timeRequest(next: any) {
	const start = Date.now();
	await next();
	const ms = Date.now() - start;
	console.log(`Request took ${ms}ms`);
}

// deno-lint-ignore no-explicit-any
function logRequest(context: any) {
	console.log(`[${context.request.method}] - ${context.request.url}`);
}

export function setupMiddleware(app: Application) {
	app.use(async (context, next) => {
		try {
			logRequest(context);
			await timeRequest(next);
		} catch (e) {
			context.response.status = e.status;
			context.response.body = e.message;
		}
	});
}


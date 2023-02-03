/* This file has been generated with "deno task create_package" */

import {radarrApi} from "../../utils/radarr.ts";

export async function getMovies() {
	return await radarrApi("GET", "/movie");
}

export async function getMovieById(id: string) {
	return await radarrApi("GET", `/movie/${id}`);
}

/* This file has been generated with "deno task create_package" */

import {sonarrApi} from "../../utils/sonarr.ts";

export async function getSonarrCalendar() {
	try {
		return await sonarrApi("GET", "/calendar");
	} catch (e) {
		// Probably has no config set
		return [];
	}
}

export async function getRadarrCalendar() {
	// ...
	return [];
}

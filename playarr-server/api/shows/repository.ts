import { sonarrApi } from "../../utils/sonarr.ts";

export async function getShows() {
	return await sonarrApi("GET", "/series");
}

export async function getEpisodesByShow(id: string) {
	return await sonarrApi("GET", `/episode?seriesId=${id}&includeImages=true`);
}

export async function getEpisodeById(id: string) {
	return await sonarrApi("GET", `/episode/${id}`);
}

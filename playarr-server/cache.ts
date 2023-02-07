import { Zoic } from "zoic";

export const cache = new Zoic({
	cache: 'LFU',
	expire: '5m',
});

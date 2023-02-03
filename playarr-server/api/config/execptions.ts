export class ConfigNotFoundException extends Error {
	constructor(name: string) {
		super(`Could not find configuration for ${name}`);
		this.name = name;
	}
}

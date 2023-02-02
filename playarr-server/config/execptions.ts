export class ConfigNotFoundException extends Error {
	constructor(name: string) {
		super(`Could not find configuration with ${name}`);
		this.name = name;
	}
}

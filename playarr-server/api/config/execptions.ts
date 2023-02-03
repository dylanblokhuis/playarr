export class ConfigNotFoundException extends Error {
	status: number;

	constructor(name: string) {
		super(`Could not find configuration for ${name}`);
		this.name = name;
		this.status = 404; // Not found
	}
}

export class ConfigHasNoValueException extends Error {
	status: number;

	constructor(name: string) {
		super(`Configuration ${name} has no value`);
		this.name = name;
		this.status = 400; // Invalid request
	}
}

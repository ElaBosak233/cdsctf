export interface Config {
	site?: {
		title?: string;
		description?: string;
	};
	container?: {
		parallel_limit?: number;
		request_limit?: number;
	};
	user?: {
		registration?: {
			enabled?: boolean;
			email?: {
				domain?: Array<string>;
				verification?: boolean;
			};
		};
	};
}

export interface ConfigUpdateRequest {
	site?: {
		title?: string;
		description?: string;
	};
	container?: {
		parallel_limit?: number;
		request_limit?: number;
	};
	user?: {
		registration?: {
			enabled?: boolean;
			email?: {
				domain?: Array<string>;
				verification?: boolean;
			};
		};
	};
}

export interface CaptchaConfig {
	enabled?: boolean;
	provider?: string;
	turnstile?: {
		site_key?: string;
	};
	recaptcha?: {
		site_key?: string;
	};
}

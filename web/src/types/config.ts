export interface Config {
	site?: {
		title?: string;
		description?: string;
		color?: string;
	};
	container?: {
		parallel_limit?: number;
		request_limit?: number;
	};
	user?: {
		register?: {
			enabled?: boolean;
			captcha?: {
				enabled?: boolean;
			};
			email?: {
				domains?: Array<string>;
				enabled?: boolean;
			};
		};
	};
}

export interface ConfigUpdateRequest {
	site?: {
		title?: string;
		description?: string;
		color?: string;
	};
	container?: {
		parallel_limit?: number;
		request_limit?: number;
	};
	user?: {
		register?: {
			enabled?: boolean;
			captcha?: {
				enabled?: boolean;
			};
			email?: {
				domains?: Array<string>;
				enabled?: boolean;
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

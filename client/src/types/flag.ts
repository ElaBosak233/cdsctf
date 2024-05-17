export interface Flag {
	id: number;
	type: string;
	banned: boolean;
	value: string;
	env: string;
	challenge_id: number;
}

export interface FlagCreateRequest {
	type: string;
	banned: boolean;
	value: string;
	env: string;
	challenge_id: number;
}

export interface FlagUpdateRequest {
	id: number;
	type: string;
	banned: boolean;
	value: string;
	env: string;
	challenge_id: number;
}

export interface FlagDeleteRequest {
	id: number;
	challenge_id: number;
}

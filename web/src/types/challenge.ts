import { Category } from "./category";
import { Flag } from "./flag";
import { Hint } from "./hint";
import { Submission } from "./submission";
import { Port } from "./port";
import { Env } from "./env";
import { File } from "./file";

export interface Challenge {
	id?: number;
	title?: string;
	description?: string;
	category_id?: number;
	category?: Category;
	attachment?: File;
	is_practicable?: boolean;
	is_dynamic?: boolean;
	difficulty?: number;
	practice_pts?: number;
	pts?: number;
	duration?: number;
	image_name?: string;
	memory_limit?: number;
	cpu_limit?: number;
	ports?: Array<Port>;
	envs?: Array<Env>;
	flags?: Array<Flag>;
	hints?: Array<Hint>;
	solved?: Submission | boolean;
	submissions?: Array<Submission>;
	is_enabled?: boolean;
	min_pts?: number;
	max_pts?: number;
}

export interface ChallengeFindRequest {
	id?: number;
	title?: string;
	description?: string;
	category_id?: number;
	is_practicable?: boolean;
	is_dynamic?: boolean;
	is_detailed?: boolean;
	difficulty?: number;
	page?: number;
	size?: number;
	submission_qty?: number;
	sort_key?: string;
	sort_order?: string;
}

export interface ChallengeUpdateRequest {
	id?: number;
	title?: string;
	description?: string;
	category_id?: number;
	attachment_url?: string;
	is_practicable?: boolean;
	is_dynamic?: boolean;
	difficulty?: number;
	practice_pts?: number;
	duration?: number;
	image_name?: string;
	memory_limit?: number;
	cpu_limit?: number;
	ports?: Array<Port>;
	envs?: Array<Env>;
}

export interface ChallengeCreateRequest {
	title?: string;
	description?: string;
	category_id?: number;
	is_practicable?: boolean;
	is_dynamic?: boolean;
	difficulty?: number;
	practice_pts?: number;
	duration?: number;
	image_name?: string;
	memory_limit?: number;
	cpu_limit?: number;
	ports?: Array<Port>;
	envs?: Array<Env>;
}

export interface ChallengeDeleteRequest {
	id?: number;
}

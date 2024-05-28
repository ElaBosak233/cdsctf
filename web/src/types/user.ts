import { Team } from "@/types/team";
import { File } from "./file";

export interface User {
	id?: number;
	username?: string;
	nickname?: string;
	email?: string;
	avatar?: File;
	group?: string;
	teams?: Array<Team>;
	created_at?: string;
	updated_at?: string;
}

export interface UserFindRequest {
	id?: number;
	name?: string;
	username?: string;
	nickname?: string;
	email?: string;
	group?: string;
	page?: number;
	size?: number;
	sort_key?: string;
	sort_order?: string;
}

export interface UserUpdateRequest {
	id: number;
	username?: string;
	nickname?: string;
	email?: string;
	group?: string;
	password?: string;
}

export interface UserCreateRequest {
	username?: string;
	nickname?: string;
	email?: string;
	group?: string;
	password?: string;
}

export interface UserDeleteRequest {
	id: number;
}

export interface UserLoginRequest {
	username: string;
	password: string;
}

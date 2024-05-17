export interface Category {
	id?: number;
	name?: string;
	description?: string;
	color?: string;
	icon?: string;
	created_at?: number;
	updated_at?: number;
}

export interface CategoryCreateRequest {
	name?: string;
	description?: string;
	color?: string;
	icon?: string;
}

export interface CategoryUpdateRequest {
	id?: number;
	name?: string;
	description?: string;
	color?: string;
	icon?: string;
}

export interface CategoryDeleteRequest {
	id?: number;
}

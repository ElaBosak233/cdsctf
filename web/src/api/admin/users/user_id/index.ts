import { Group, User } from "@/models/user";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface DeleteUserRequest {
    id: number;
}

export async function deleteUser(request: DeleteUserRequest) {
    return alova.Delete<WebResponse<never>>(
        `/admin/users/${request.id}`,
        request
    );
}

export interface UpdateUserRequest {
    id: number;
    username?: string;
    nickname?: string;
    email?: string;
    group?: Group;
    password?: string;
    is_verified?: boolean;
    description?: string;
}

export async function updateUser(request: UpdateUserRequest) {
    return alova.Put<WebResponse<User>>(`/admin/users/${request.id}`, request);
}

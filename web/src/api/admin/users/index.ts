import { Group, User } from "@/models/user";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface CreateUserRequest {
    username?: string;
    nickname?: string;
    email?: string;
    group?: Group;
    password?: string;
}

export async function createUser(request: CreateUserRequest) {
    return alova.Post<WebResponse<User>>("/admin/users", request);
}

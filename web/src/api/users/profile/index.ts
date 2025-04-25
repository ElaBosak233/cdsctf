import { User } from "@/models/user";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface UpdateUserProfileRequest {
    nickname?: string;
    email?: string;
    description?: string | null;
}

export async function updateUserProfile(request: UpdateUserProfileRequest) {
    return alova.Put<WebResponse<User>>("/users/profile", request);
}

export interface UpdateUserProfilePasswordRequest {
    old_password: string;
    new_password: string;
}

export async function updateUserProfilePassword(
    request: UpdateUserProfilePasswordRequest
) {
    return alova.Put<WebResponse<never>>("/users/profile/password", request);
}

export interface DeleteUserProfileRequest {
    password: string;
    captcha?: {
        id?: string;
        content?: string;
    } | null;
}

export async function deleteUserProfile(request: DeleteUserProfileRequest) {
    return alova.Delete<WebResponse<never>>("/users/profile", request);
}

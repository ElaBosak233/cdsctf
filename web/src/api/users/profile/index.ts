import { User } from "@/models/user";
import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function getUserProfile() {
  return api
    .get("users/profile", {
      headers: {
        "ignore-unauthorized": "OK",
      },
    })
    .json<WebResponse<User>>();
}

export interface UpdateUserProfileRequest {
  name?: string;
  email?: string;
  description?: string | null;
}

export async function updateUserProfile(request: UpdateUserProfileRequest) {
  return api.put("users/profile", { json: request }).json<WebResponse<User>>();
}

export interface UpdateUserProfilePasswordRequest {
  old_password: string;
  new_password: string;
}

export async function updateUserProfilePassword(
  request: UpdateUserProfilePasswordRequest
) {
  return api
    .put("users/profile/password", { json: request })
    .json<WebResponse<never>>();
}

export interface DeleteUserProfileRequest {
  password: string;
  captcha?: {
    id?: string;
    content?: string;
  } | null;
}

export async function deleteUserProfile(request: DeleteUserProfileRequest) {
  return api
    .delete("users/profile", { json: request })
    .json<WebResponse<never>>();
}

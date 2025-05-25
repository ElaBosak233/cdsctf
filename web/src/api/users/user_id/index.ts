import { User } from "@/models/user";
import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface GetUserRequest {
  id: number;
}

export async function getUser(request: GetUserRequest) {
  return api.get(`users/${request.id}`).json<WebResponse<User>>();
}

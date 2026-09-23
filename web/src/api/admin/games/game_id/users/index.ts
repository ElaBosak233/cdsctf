import type { UserAccountView } from "@/models/user";
import { api, toSearchParams } from "@/utils/query";

export type GetGameUsersRequest = {
  game_id?: number;
  id?: number;
  name?: string;
  page?: number;
  size?: number;
  sorts?: string;
};

export async function getGameUsers(request: GetGameUsersRequest) {
  return api
    .get(`admin/games/${request.game_id}/users`, {
      searchParams: toSearchParams(request),
    })
    .json<{ users: UserAccountView[]; total: number }>();
}

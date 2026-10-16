import type { UserStatistics } from "@/models/user";
import { api } from "@/utils/query";

export async function getUserStatistics(id: number) {
  return api
    .get(`users/${id}/statistics`)
    .json<{ statistics: UserStatistics }>();
}

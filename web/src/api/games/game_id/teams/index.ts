import type { Team } from "@/models/team";
import { api } from "@/utils/query";

export interface CreateTeamRequest {
  game_id?: number;
  name?: string;
  email?: string;
  slogan?: string;
  description?: string;
}

export async function createTeam(request: CreateTeamRequest) {
  return api
    .post(`games/${request.game_id}/teams`, {
      json: {
        name: request.name,
        email: request.email,
        slogan: request.slogan,
        description: request.description,
      },
    })
    .json<{ team: Team }>();
}

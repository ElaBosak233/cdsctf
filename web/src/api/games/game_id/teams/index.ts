import { Team } from "@/models/team";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

// export interface GetTeamRequest {
//     id?: number;
//     game_id?: number;
//     user_id?: number;
//     name?: string;
//     state?: State;
//     page?: number;
//     size?: number;
//     sorts?: string;
// }

// export async function getTeams(request: GetTeamRequest) {
//     return alova.Get<WebResponse<Array<Team>>>(
//         `/games/${request.game_id}/teams`,
//         {
//             params: request,
//         }
//     );
// }

export interface TeamRegisterRequest {
    game_id?: number;
    name?: string;
    email?: string;
    slogan?: string;
    description?: string;
}

export async function teamRegister(request: TeamRegisterRequest) {
    return alova.Post<WebResponse<Team>>(
        `/games/${request.game_id}/teams/register`,
        request
    );
}

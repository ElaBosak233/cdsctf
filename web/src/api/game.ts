import {
    Game,
    GameChallengeFindRequest,
    GameCreateRequest,
    GameDeleteRequest,
    GameFindRequest,
    GameSubmissionGetRequest,
    GameTeamCreateRequest,
    GameTeamDeleteRequest,
    GameTeamFindRequest,
    GameTeamUpdateRequest,
    GameUpdateRequest,
} from "@/types/game";
import {
    GameChallenge,
    GameChallengeCreateRequest,
    GameChallengeDeleteRequest,
    GameChallengeUpdateRequest,
} from "@/types/game_challenge";
import { GameTeam } from "@/types/game_team";
import {
    Notice,
    NoticeCreateRequest,
    NoticeDeleteRequest,
    NoticeFindRequest,
    NoticeUpdateRequest,
} from "@/types/notice";
import { GameSubmission } from "@/types/submission";
import { api } from "@/utils/axios";
import { AxiosRequestConfig } from "axios";

export async function getGames(request: GameFindRequest) {
    return api().get<{
        code: number;
        data: Array<Game>;
        total: number;
    }>("/games", { params: request });
}

export async function createGame(request: GameCreateRequest) {
    return api().post<{
        code: number;
        data: Game;
    }>("/games", request);
}

export async function updateGame(request: GameUpdateRequest) {
    return api().put<{
        code: number;
        data: Game;
    }>(`/games/${request?.id}`, request);
}

export async function deleteGame(request: GameDeleteRequest) {
    return api().delete<{
        code: number;
    }>(`/games/${request?.id}`);
}

export async function getGameChallenges(request: GameChallengeFindRequest) {
    return api().get<{
        code: number;
        data: Array<GameChallenge>;
    }>(`/games/${request?.game_id}/challenges`, { params: request });
}

export async function createGameChallenge(request: GameChallengeCreateRequest) {
    return api().post<{
        code: number;
        data: GameChallenge;
    }>(`/games/${request?.game_id}/challenges`, request);
}

export async function updateGameChallenge(request: GameChallengeUpdateRequest) {
    return api().put<{
        code: number;
        data: GameChallenge;
    }>(
        `/games/${request?.game_id}/challenges/${request?.challenge_id}`,
        request
    );
}

export async function deleteGameChallenge(request: GameChallengeDeleteRequest) {
    return api().delete<{
        code: number;
    }>(`/games/${request?.game_id}/challenges/${request?.challenge_id}`);
}

export async function getGameTeams(request: GameTeamFindRequest) {
    return api().get<{
        code: number;
        data: Array<GameTeam>;
        total: number;
    }>(`/games/${request?.game_id}/teams`, { params: request });
}

export async function createGameTeam(request: GameTeamCreateRequest) {
    return api().post<{
        code: number;
        data: GameTeam;
    }>(`/games/${request?.game_id}/teams`, request);
}

export async function deleteGameTeam(request: GameTeamDeleteRequest) {
    return api().delete<{
        code: number;
    }>(`/games/${request?.game_id}/teams/${request?.team_id}`);
}

export async function updateGameTeam(request: GameTeamUpdateRequest) {
    return api().put<{
        code: number;
        data: GameTeam;
    }>(`/games/${request?.game_id}/teams/${request?.team_id}`, request);
}

export async function getGameSubmissions(request: GameSubmissionGetRequest) {
    return api().get<{
        code: number;
        data: Array<GameSubmission>;
    }>(`/games/${request?.id}/submissions`, { params: request });
}

export async function getGameNotices(request: NoticeFindRequest) {
    return api().get<{
        code: number;
        data: Array<Notice>;
        total: number;
    }>(`/games/${request?.game_id}/notices`, { params: request });
}

export async function createGameNotice(request: NoticeCreateRequest) {
    return api().post<{
        code: number;
        data: Notice;
    }>(`/games/${request?.game_id}/notices`, request);
}

export async function updateGameNotice(request: NoticeUpdateRequest) {
    return api().put<{
        code: number;
        data: Notice;
    }>(`/games/${request?.game_id}/notices/${request?.id}`, request);
}

export async function deleteGameNotice(request: NoticeDeleteRequest) {
    return api().delete<{
        code: number;
    }>(`/games/${request?.game_id}/notices/${request?.id}`);
}

export async function getGamePosterMetadata(id: number) {
    return api().get<{
        code: number;
        data: {
            filename: string;
            size: number;
        };
    }>(`/games/${id}/poster/metadata`);
}

export async function saveGamePoster(
    id: number,
    file: File,
    config: AxiosRequestConfig<FormData>
) {
    const formData = new FormData();
    formData.append("file", file);
    return api().post(`/games/${id}/poster`, formData, config);
}

export async function deleteGamePoster(id: number) {
    return api().delete<{
        code: number;
    }>(`/games/${id}/poster`);
}

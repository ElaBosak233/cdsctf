import {
    Team,
    TeamCreateRequest,
    TeamDeleteRequest,
    TeamDeleteUserRequest,
    TeamFindRequest,
    TeamGetInviteTokenRequest,
    TeamJoinRequest,
    TeamLeaveRequest,
    TeamUpdateInviteTokenRequest,
    TeamUpdateRequest,
} from "@/types/team";
import { api } from "@/utils/axios";
import { AxiosRequestConfig } from "axios";

export async function getTeams(request: TeamFindRequest) {
    return api().get<{
        code: number;
        data: Array<Team>;
        total: number;
    }>("/teams", { params: request });
}

export async function createTeam(request: TeamCreateRequest) {
    return api().post<{
        code: number;
        data: Team;
    }>("/teams", request);
}

export async function updateTeam(request: TeamUpdateRequest) {
    return api().put<{
        code: number;
        data: Team;
    }>(`/teams/${request?.id}`, request);
}

export async function deleteTeam(request: TeamDeleteRequest) {
    return api().delete<{
        code: number;
        data: Team;
    }>(`/teams/${request?.id}`);
}

export async function deleteTeamUser(request: TeamDeleteUserRequest) {
    return api().delete<{
        code: number;
        data: Team;
    }>(`/teams/${request?.id}/users/${request?.user_id}`);
}

export async function getTeamInviteToken(request: TeamGetInviteTokenRequest) {
    return api().get<{
        code: number;
        token: string;
    }>(`/teams/${request?.id}/invite`);
}

export async function updateTeamInviteToken(
    request: TeamUpdateInviteTokenRequest
) {
    return api().put<{
        code: number;
        token: string;
    }>(`/teams/${request?.id}/invite`, request);
}

export async function joinTeam(request: TeamJoinRequest) {
    return api().post<{
        code: number;
        data: Team;
    }>(`/teams/${request?.id}/join`, request);
}

export async function leaveTeam(request: TeamLeaveRequest) {
    return api().delete<{
        code: number;
        data: Team;
    }>(`/teams/${request?.id}/leave`);
}

export async function getTeamAvatarMetadata(id: number) {
    return api().get<{
        code: number;
        data: {
            filename: string;
            size: number;
        };
    }>(`/teams/${id}/avatar/metadata`);
}

export async function saveTeamAvatar(
    id: number,
    file: File,
    config: AxiosRequestConfig<FormData>
) {
    const formData = new FormData();
    formData.append("file", file);
    return api().post<{
        code: number;
        data: {
            url: string;
        };
    }>(`/teams/${id}/avatar`, formData, config);
}

export async function deleteTeamAvatar(id: number) {
    return api().delete<{
        code: number;
    }>(`/teams/${id}/avatar`);
}

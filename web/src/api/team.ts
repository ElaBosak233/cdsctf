import {
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
import { useAuth } from "@/utils/axios";
import { AxiosRequestConfig } from "axios";

export function useTeamApi() {
    const auth = useAuth();

    const getTeams = (request?: TeamFindRequest) => {
        return auth.get("/teams", { params: request });
    };

    const createTeam = (request?: TeamCreateRequest) => {
        return auth.post("/teams", request);
    };

    const updateTeam = (request: TeamUpdateRequest) => {
        return auth.put(`/teams/${request?.id}`, request);
    };

    const deleteTeam = (request: TeamDeleteRequest) => {
        return auth.delete(`/teams/${request?.id}`);
    };

    const deleteTeamUser = (request: TeamDeleteUserRequest) => {
        return auth.delete(`/teams/${request?.id}/users/${request?.user_id}`);
    };

    const getTeamInviteToken = (request: TeamGetInviteTokenRequest) => {
        return auth.get(`/teams/${request?.id}/invite`);
    };

    const updateTeamInviteToken = (request: TeamUpdateInviteTokenRequest) => {
        return auth.put(`/teams/${request?.id}/invite`, request);
    };

    const joinTeam = (request: TeamJoinRequest) => {
        return auth.post(`/teams/${request?.id}/join`, request);
    };

    const leaveTeam = (request: TeamLeaveRequest) => {
        return auth.delete(`/teams/${request?.id}/leave`);
    };

    const getTeamAvatarMetadata = (id: number) => {
        return auth.get(`/teams/${id}/avatar/metadata`);
    };

    const saveTeamAvatar = (
        id: number,
        file: File,
        config: AxiosRequestConfig<FormData>
    ) => {
        const formData = new FormData();
        formData.append("file", file);
        return auth.post(`/teams/${id}/avatar`, formData, config);
    };

    const deleteTeamAvatar = (id: number) => {
        return auth.delete(`/teams/${id}/avatar`);
    };

    return {
        getTeams,
        createTeam,
        deleteTeam,
        deleteTeamUser,
        updateTeam,
        joinTeam,
        leaveTeam,
        getTeamInviteToken,
        updateTeamInviteToken,
        getTeamAvatarMetadata,
        saveTeamAvatar,
        deleteTeamAvatar,
    };
}

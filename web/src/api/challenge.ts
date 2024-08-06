import {
    Challenge,
    ChallengeCreateRequest,
    ChallengeDeleteRequest,
    ChallengeFindRequest,
    ChallengeStatus,
    ChallengeStatusRequest,
    ChallengeUpdateRequest,
} from "@/types/challenge";
import { api } from "@/utils/axios";
import { AxiosRequestConfig } from "axios";

export async function getChallenges(request: ChallengeFindRequest) {
    return await api().get<{
        code: number;
        data: Array<Challenge>;
        total: number;
    }>("/challenges", { params: request });
}

export async function createChallenge(request: ChallengeCreateRequest) {
    return await api().post<{
        code: number;
        data: Challenge;
    }>("/challenges", request);
}

export async function updateChallenge(request: ChallengeUpdateRequest) {
    return await api().put<{
        code: number;
        data: Challenge;
    }>(`/challenges/${request.id}`, request);
}

export async function deleteChallenge(request: ChallengeDeleteRequest) {
    return await api().delete<{
        code: number;
    }>(`/challenges/${request.id}`);
}

export async function getChallengeStatus(request: ChallengeStatusRequest) {
    return await api().post<{
        code: number;
        data: Record<number, ChallengeStatus>;
    }>("/challenges/status", request);
}

export async function getChallengeAttachmentMetadata(id: number) {
    return await api().get<{
        code: number;
        data: {
            filename: string;
            size: number;
        };
    }>(`/challenges/${id}/attachment/metadata`);
}

export async function saveChallengeAttachment(
    id: number,
    file: File,
    config: AxiosRequestConfig<FormData>
) {
    const formData = new FormData();
    formData.append("file", file);
    return await api().post<{
        code: number;
    }>(`/challenges/${id}/attachment`, formData, config);
}

export async function deleteChallengeAttachment(id: number) {
    return await api().delete<{
        code: number;
    }>(`/challenges/${id}/attachment`);
}

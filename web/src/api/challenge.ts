import {
    ChallengeCreateRequest,
    ChallengeDeleteRequest,
    ChallengeFindRequest,
    ChallengeStatusRequest,
    ChallengeUpdateRequest,
} from "@/types/challenge";
import { useAuth } from "@/utils/axios";
import { AxiosRequestConfig } from "axios";

export function useChallengeApi() {
    const auth = useAuth();

    const getChallenges = (request: ChallengeFindRequest) => {
        return auth.get("/challenges", { params: request });
    };

    const createChallenge = (request: ChallengeCreateRequest) => {
        return auth.post("/challenges", request);
    };

    const updateChallenge = (request: ChallengeUpdateRequest) => {
        return auth.put(`/challenges/${request.id}`, request);
    };

    const deleteChallenge = (request: ChallengeDeleteRequest) => {
        return auth.delete(`/challenges/${request.id}`);
    };

    const getChallengeStatus = (request: ChallengeStatusRequest) => {
        return auth.post(`/challenges/status`, request);
    };

    const getChallengeAttachmentMetadata = (id: number) => {
        return auth.get(`/challenges/${id}/attachment/metadata`);
    };

    const saveChallengeAttachment = (
        id: number,
        file: File,
        config: AxiosRequestConfig<FormData>
    ) => {
        const formData = new FormData();
        formData.append("file", file);
        return auth.post(`/challenges/${id}/attachment`, formData, config);
    };

    const deleteChallengeAttachment = (id: number) => {
        return auth.delete(`/challenges/${id}/attachment`);
    };

    return {
        getChallenges,
        createChallenge,
        updateChallenge,
        deleteChallenge,
        getChallengeStatus,
        getChallengeAttachmentMetadata,
        saveChallengeAttachment,
        deleteChallengeAttachment,
    };
}

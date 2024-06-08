import {
	ChallengeCreateRequest,
	ChallengeDeleteRequest,
	ChallengeFindRequest,
	ChallengeUpdateRequest,
} from "@/types/challenge";
import {
	FlagCreateRequest,
	FlagDeleteRequest,
	FlagUpdateRequest,
} from "@/types/flag";
import { useAuth } from "@/utils/axios";
import { AxiosRequestConfig } from "axios";

export function useChallengeApi() {
	const auth = useAuth();

	const getChallenges = (request: ChallengeFindRequest) => {
		return auth.get("/challenges/", { params: request });
	};

	const createChallenge = (request: ChallengeCreateRequest) => {
		return auth.post("/challenges/", request);
	};

	const updateChallenge = (request: ChallengeUpdateRequest) => {
		return auth.put(`/challenges/${request.id}`, request);
	};

	const deleteChallenge = (request: ChallengeDeleteRequest) => {
		return auth.delete(`/challenges/${request.id}`);
	};

	const updateChallengeFlag = (request: FlagUpdateRequest) => {
		return auth.put(
			`/challenges/${request.challenge_id}/flags/${request.id}`,
			request
		);
	};

	const createChallengeFlag = (request: FlagCreateRequest) => {
		return auth.post(`/challenges/${request.challenge_id}/flags`, request);
	};

	const deleteChallengeFlag = (request: FlagDeleteRequest) => {
		return auth.delete(
			`/challenges/${request.challenge_id}/flags/${request.id}`
		);
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
		updateChallengeFlag,
		createChallengeFlag,
		deleteChallengeFlag,
		saveChallengeAttachment,
		deleteChallengeAttachment,
	};
}

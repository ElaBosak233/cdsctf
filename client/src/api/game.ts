import {
	GameChallengeCreateRequest,
	GameChallengeFindRequest,
	GameChallengeUpdateRequest,
	GameCreateRequest,
	GameDeleteRequest,
	GameFindRequest,
	GameTeamCreateRequest,
	GameTeamDeleteRequest,
	GameTeamFindRequest,
	GameTeamUpdateRequest,
	GameUpdateRequest,
} from "@/types/game";
import {
	NoticeCreateRequest,
	NoticeDeleteRequest,
	NoticeFindRequest,
	NoticeUpdateRequest,
} from "@/types/notice";
import { useAuth } from "@/utils/axios";
import { AxiosRequestConfig } from "axios";

export function useGameApi() {
	const auth = useAuth();

	const getGames = (request: GameFindRequest) => {
		return auth.get("/games/", { params: request });
	};

	const getGameByID = (id: number) => {
		return auth.get(`/games/${id}`);
	};

	const createGame = (request: GameCreateRequest) => {
		return auth.post("/games/", request);
	};

	const updateGame = (request: GameUpdateRequest) => {
		return auth.put(`/games/${request?.id}`, request);
	};

	const deleteGame = (request: GameDeleteRequest) => {
		return auth.delete(`/games/${request?.id}`);
	};

	const getGameChallenges = (request: GameChallengeFindRequest) => {
		return auth.get(`/games/${request?.game_id}/challenges`, {
			params: request,
		});
	};

	const createGameChallenge = (request: GameChallengeCreateRequest) => {
		return auth.post(`/games/${request?.game_id}/challenges`, request);
	};

	const updateGameChallenge = (request: GameChallengeUpdateRequest) => {
		return auth.put(
			`/games/${request?.game_id}/challenges/${request?.challenge_id}`,
			request
		);
	};

	const deleteGameChallenge = (request: GameChallengeUpdateRequest) => {
		return auth.delete(
			`/games/${request?.game_id}/challenges/${request?.challenge_id}`
		);
	};

	const getGameTeams = (request: GameTeamFindRequest) => {
		return auth.get(`/games/${request?.game_id}/teams`, {
			params: request,
		});
	};

	const getGameTeamByID = (request: GameTeamFindRequest) => {
		return auth.get(`/games/${request?.game_id}/teams/${request?.team_id}`);
	};

	const createGameTeam = (request: GameTeamCreateRequest) => {
		return auth.post(`/games/${request?.game_id}/teams`, request);
	};

	const updateGameTeam = (request: GameTeamUpdateRequest) => {
		return auth.put(
			`/games/${request?.game_id}/teams/${request?.team_id}`,
			request
		);
	};

	const deleteGameTeam = (request: GameTeamDeleteRequest) => {
		return auth.delete(
			`/games/${request?.game_id}/teams/${request?.team_id}`
		);
	};

	const getGameNotices = (request: NoticeFindRequest) => {
		return auth.get(`/games/${request?.game_id}/notices`, {
			params: request,
		});
	};

	const createGameNotice = (request: NoticeCreateRequest) => {
		return auth.post(`/games/${request?.game_id}/notices`, request);
	};

	const updateGameNotice = (request: NoticeUpdateRequest) => {
		return auth.put(
			`/games/${request?.game_id}/notices/${request?.id}`,
			request
		);
	};

	const deleteGameNotice = (request: NoticeDeleteRequest) => {
		return auth.delete(`/games/${request?.game_id}/notices/${request?.id}`);
	};

	const saveGamePoster = (
		id: number,
		file: File,
		config: AxiosRequestConfig<FormData>
	) => {
		const formData = new FormData();
		formData.append("file", file);
		return auth.post(`/games/${id}/poster`, formData, config);
	};

	const deleteGamePoster = (id: number) => {
		return auth.delete(`/games/${id}/poster`);
	};

	return {
		getGames,
		getGameByID,
		getGameChallenges,
		createGame,
		updateGame,
		deleteGame,
		updateGameChallenge,
		createGameChallenge,
		deleteGameChallenge,
		getGameTeams,
		getGameTeamByID,
		createGameTeam,
		updateGameTeam,
		deleteGameTeam,
		getGameNotices,
		createGameNotice,
		updateGameNotice,
		deleteGameNotice,
		saveGamePoster,
		deleteGamePoster,
	};
}

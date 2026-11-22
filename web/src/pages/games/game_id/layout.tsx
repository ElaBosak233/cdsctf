import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { useEffect, useState } from "react";
import { Outlet, useParams } from "react-router";
import { getGame } from "@/api/games/game_id";
import { getTeamMembers } from "@/api/games/game_id/teams/team_id";
import { getTeamProfile } from "@/api/games/game_id/teams/us";
import { BASIC_PERMISSION, hasPermission } from "@/models/user";
import { useAuthStore } from "@/storages/auth";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "./context";

function useGameQuery(gameId: number | undefined, trigger: number = 0) {
  return useQuery({
    queryKey: ["game", trigger, gameId],
    queryFn: () => getGame({ id: gameId! }),
    select: (response) => response.game,
    enabled: gameId != null,
    placeholderData: keepPreviousData,
    refetchInterval: 5000,
  });
}

export default function GameLayout() {
  const { game_id } = useParams<{ game_id: string }>();
  const gameId = parseRouteNumericId(game_id);
  const { setCurrentGame, selfTeam, setSelfTeam, setMembers } = useGameStore();
  const sharedStore = useSharedStore();
  const { status: authStatus, user } = useAuthStore();
  const canPlay =
    authStatus === "authenticated" &&
    hasPermission(user?.permissions, BASIC_PERMISSION);

  const [gtLoaded, setGtLoaded] = useState<boolean>(false);

  const { data: game } = useGameQuery(gameId);

  const teamProfileQuery = useQuery({
    queryKey: ["game", "team-profile", gameId, user?.id, sharedStore?.refresh],
    queryFn: () => getTeamProfile({ game_id: gameId! }),
    enabled: gameId != null && canPlay,
    retry: false,
    meta: { suppressErrorToast: true },
  });

  const teamMembersQuery = useQuery({
    queryKey: [
      "game",
      "team-members",
      user?.id,
      gameId,
      selfTeam?.id,
      sharedStore?.refresh,
    ],
    queryFn: () => getTeamMembers({ game_id: gameId!, team_id: selfTeam!.id }),
    enabled: gameId != null && canPlay && selfTeam?.id != null,
    retry: false,
  });

  useEffect(() => {
    if (game_id !== useGameStore.getState().currentGame?.id) {
      setCurrentGame(undefined);
    }

    setCurrentGame(game);
  }, [game_id, game, setCurrentGame]);

  useEffect(() => {
    if (game?.blacked_out) {
      const currentTeam = useGameStore.getState().selfTeam;
      if (
        currentTeam &&
        (currentTeam.pts !== undefined || currentTeam.rank !== undefined)
      ) {
        setSelfTeam({ ...currentTeam, pts: undefined, rank: undefined });
      }
    }
  }, [game?.blacked_out, setSelfTeam]);

  useEffect(() => {
    if (!canPlay) {
      setSelfTeam(undefined);
      setMembers([]);
    } else if (teamProfileQuery.data?.team) {
      setSelfTeam(teamProfileQuery.data.team);
    } else if (
      teamProfileQuery.error instanceof HTTPError &&
      teamProfileQuery.error.response.status === StatusCodes.NOT_FOUND
    ) {
      setSelfTeam(undefined);
    }
  }, [
    canPlay,
    teamProfileQuery.data,
    teamProfileQuery.error,
    setSelfTeam,
    setMembers,
  ]);

  useEffect(() => {
    setGtLoaded(gameId == null || !canPlay || teamProfileQuery.isFetched);
  }, [canPlay, gameId, teamProfileQuery.isFetched]);

  useEffect(() => {
    if (canPlay && teamMembersQuery.data) {
      setMembers(teamMembersQuery.data.users);
    }
  }, [canPlay, setMembers, teamMembersQuery.data]);

  return (
    <Context.Provider value={{ gtLoaded }}>
      <Outlet />
    </Context.Provider>
  );
}

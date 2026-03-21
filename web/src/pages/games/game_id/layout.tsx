import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { useEffect, useState } from "react";
import { Outlet, useParams } from "react-router";
import { getGame } from "@/api/games/game_id";
import { getTeamMembers } from "@/api/games/game_id/teams/team_id";
import { getTeamProfile } from "@/api/games/game_id/teams/us";
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
  });
}

export default function GameLayout() {
  const { game_id } = useParams<{ game_id: string }>();
  const gameId = parseRouteNumericId(game_id);
  const { setCurrentGame, selfTeam, setSelfTeam, setMembers } = useGameStore();
  const sharedStore = useSharedStore();
  const authStore = useAuthStore();

  const [gtLoaded, setGtLoaded] = useState<boolean>(false);

  const { data: game } = useGameQuery(gameId);

  useEffect(() => {
    if (game_id !== useGameStore.getState().currentGame?.id) {
      setCurrentGame(undefined);
    }

    setCurrentGame(game);
  }, [game_id, game, setCurrentGame]);

  useEffect(() => {
    void sharedStore?.refresh;

    if (gameId == null) {
      setGtLoaded(true);
      return;
    }

    if (!authStore?.user) return;

    (async () => {
      try {
        const res = await getTeamProfile({
          game_id: gameId,
        });
        setSelfTeam(res.team);
      } catch (error) {
        if (!(error instanceof HTTPError)) return;

        if (error.response.status === StatusCodes.NOT_FOUND) {
          setSelfTeam(undefined);
        }
      } finally {
        setGtLoaded(true);
      }
    })();
  }, [sharedStore?.refresh, gameId, setSelfTeam, authStore?.user]);

  useEffect(() => {
    void sharedStore?.refresh;

    if (!selfTeam?.id || gameId == null) return;

    getTeamMembers({
      game_id: gameId,
      team_id: selfTeam.id,
    }).then((res) => {
      setMembers(res.users);
    });
  }, [sharedStore?.refresh, selfTeam, gameId, setMembers]);

  return (
    <Context.Provider value={{ gtLoaded }}>
      <Outlet />
    </Context.Provider>
  );
}

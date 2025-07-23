import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { useEffect, useState } from "react";
import { Outlet, useParams } from "react-router";
import { getGame, type GetGameRequest } from "@/api/games/game_id";
import { getTeamProfile } from "@/api/games/game_id/teams/profile";
import { getTeamMembers } from "@/api/games/game_id/teams/team_id";
import { useAuthStore } from "@/storages/auth";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { parseErrorResponse } from "@/utils/query";
import { Context } from "./context";
import { keepPreviousData, useQuery } from "@tanstack/react-query";

function useGameQuery(params: GetGameRequest, trigger: number = 0) {
  return useQuery({
    queryKey: ["game", trigger, params.id],
    queryFn: () => getGame(params),
    select: (response) => response.data,
    enabled: !!params,
    placeholderData: keepPreviousData,
  });
}

export default function () {
  const { game_id } = useParams<{ game_id: string }>();
  const { setCurrentGame, selfTeam, setSelfTeam, setMembers } = useGameStore();
  const sharedStore = useSharedStore();
  const authStore = useAuthStore();

  const [gtLoaded, setGtLoaded] = useState<boolean>(false);

  const { data: game } = useGameQuery({ id: Number(game_id) });

  useEffect(() => {
    if (game_id !== useGameStore.getState().currentGame?.id) {
      setCurrentGame(undefined);
    }

    setCurrentGame(game);
  }, [game_id, game, setCurrentGame]);

  useEffect(() => {
    void sharedStore?.refresh;

    if (!authStore?.user) return;

    (async () => {
      try {
        const res = await getTeamProfile({
          game_id: Number(game_id),
        });
        setSelfTeam(res.data);
      } catch (error) {
        if (!(error instanceof HTTPError)) return;
        const res = await parseErrorResponse(error);

        if (res.code === StatusCodes.NOT_FOUND) {
          setSelfTeam(undefined);
        }
      } finally {
        setGtLoaded(true);
      }
    })();
  }, [sharedStore?.refresh, game_id, setSelfTeam, authStore?.user]);

  useEffect(() => {
    void sharedStore?.refresh;

    if (!selfTeam?.id) return;

    getTeamMembers({
      game_id: Number(game_id),
      team_id: selfTeam?.id,
    }).then((res) => {
      setMembers(res.data);
    });
  }, [sharedStore?.refresh, selfTeam, game_id, setMembers]);

  return (
    <Context.Provider value={{ gtLoaded }}>
      <Outlet />
    </Context.Provider>
  );
}

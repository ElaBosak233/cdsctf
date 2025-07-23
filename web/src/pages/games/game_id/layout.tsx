import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { useEffect, useState } from "react";
import { Outlet, useParams } from "react-router";
import { getGame } from "@/api/games/game_id";
import { getTeamProfile } from "@/api/games/game_id/teams/profile";
import { getTeamMembers } from "@/api/games/game_id/teams/team_id";
import { useAuthStore } from "@/storages/auth";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { parseErrorResponse } from "@/utils/query";
import { Context } from "./context";

export default function () {
  const { game_id } = useParams<{ game_id: string }>();
  const { currentGame, setCurrentGame, selfTeam, setSelfTeam, setMembers } =
    useGameStore();
  const sharedStore = useSharedStore();
  const authStore = useAuthStore();

  const [gtLoaded, setGtLoaded] = useState<boolean>(false);

  useEffect(() => {
    if (game_id !== currentGame?.id) {
      setCurrentGame(undefined);
    }

    getGame({
      id: Number(game_id),
    }).then((res) => {
      setCurrentGame(res.data);
    });
  }, [game_id, currentGame?.id, setCurrentGame]);

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

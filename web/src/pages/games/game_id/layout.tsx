import { useEffect, useState } from "react";
import { Outlet, useParams } from "react-router";

import { Context } from "./context";

import { getGame } from "@/api/games/game_id";
import { getTeamProfile } from "@/api/games/game_id/teams/profile";
import { getTeamMembers } from "@/api/games/game_id/teams/team_id";
import { useAuthStore } from "@/storages/auth";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";

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
  }, [game_id]);

  useEffect(() => {
    if (authStore?.user) {
      getTeamProfile({
        game_id: Number(game_id),
      })
        .then((res) => {
          setSelfTeam(res.data);
        })
        .finally(() => {
          setGtLoaded(true);
        });
    }
  }, [sharedStore?.refresh, game_id]);

  useEffect(() => {
    if (selfTeam?.id) {
      getTeamMembers({
        game_id: Number(game_id),
        team_id: selfTeam?.id,
      }).then((res) => {
        setMembers(res.data);
      });
    }
  }, [sharedStore?.refresh, selfTeam]);

  return (
    <Context.Provider value={{ gtLoaded }}>
      <Outlet />
    </Context.Provider>
  );
}

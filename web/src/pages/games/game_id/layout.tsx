import { Outlet, useParams } from "react-router";
import { Context } from "./context";
import { useSharedStore } from "@/storages/shared";
import { useEffect, useState } from "react";
import { getTeamProfile } from "@/api/games/game_id/teams/profile";
import { useAuthStore } from "@/storages/auth";
import { useGameStore } from "@/storages/game";
import { getGames } from "@/api/games";

export default function () {
    const { game_id } = useParams<{ game_id: string }>();
    const { currentGame, setCurrentGame, setSelfTeam } = useGameStore();
    const sharedStore = useSharedStore();
    const authStore = useAuthStore();

    const [gtLoaded, setGtLoaded] = useState<boolean>(false);

    useEffect(() => {
        if (game_id !== currentGame?.id) {
            setCurrentGame(undefined);
        }

        getGames({
            id: Number(game_id),
        }).then((res) => {
            setCurrentGame(res.data?.[0]);
        });
    }, [game_id]);

    function fetchSelfTeam() {
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

    useEffect(() => {
        if (authStore?.user) {
            fetchSelfTeam();
        }
    }, [sharedStore?.refresh, game_id]);

    return (
        <Context.Provider value={{ gtLoaded }}>
            <Outlet />
        </Context.Provider>
    );
}

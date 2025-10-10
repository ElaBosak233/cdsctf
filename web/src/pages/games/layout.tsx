import { useState } from "react";
import { Outlet } from "react-router";
import type { GameMini } from "@/models/game";
import Entrance from "./entrance";

export default function () {
  const [entranceGame, setEntranceGame] = useState<GameMini | undefined>(
    undefined
  );

  return (
    <>
      <Outlet context={{ setEntranceGame }} />
      <Entrance
        game={entranceGame}
        onFinish={() => setEntranceGame(undefined)}
      />
    </>
  );
}

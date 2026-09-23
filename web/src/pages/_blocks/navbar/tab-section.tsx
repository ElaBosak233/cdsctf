import { useContext } from "react";
import { Link, useLocation } from "react-router";
import { Button } from "@/components/ui/button";
import { useGameStore } from "@/storages/game";
import { isSubRoute } from "@/utils/route";
import { Context, useOptions } from "./context";

function TabSection() {
  const { mode } = useContext(Context);
  const { currentGame } = useGameStore();

  const location = useLocation();
  const pathname = location.pathname;
  const options = useOptions();

  return (
    <>
      {options?.map((option, index) => {
        return (
          <Button
            key={index}
            render={option?.disabled ? undefined : <Link to={option.link} />}
            variant={
              isSubRoute(
                option.link,
                pathname,
                mode === "game" ? `/games/${currentGame?.id}` : "/"
              )
                ? "tonal"
                : "ghost"
            }
            size={"sm"}
            className={"font-semibold"}
            disabled={option?.disabled}
            icon={option.icon}
            level={option?.warning ? "warning" : "primary"}
          >
            {option?.name}
          </Button>
        );
      })}
    </>
  );
}

export { TabSection };

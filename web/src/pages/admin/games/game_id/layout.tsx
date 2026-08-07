import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  CirclePauseIcon,
  FlagIcon,
  InfoIcon,
  LibraryIcon,
  MessageCircleIcon,
  MoonIcon,
  RefreshCwIcon,
  UsersRoundIcon,
} from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link, Outlet, useLocation, useParams } from "react-router";
import { toast } from "sonner";
import { getGame, updateGame } from "@/api/admin/games/game_id";
import { calculateGame } from "@/api/admin/games/game_id/calculate";
import { Button, buttonVariants } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { ScrollableNav } from "@/components/ui/scrollable-nav";
import { Switch } from "@/components/ui/switch";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "./context";

export default function Layout() {
  const { t } = useTranslation();

  const location = useLocation();
  const pathname = location.pathname;
  const sharedStore = useSharedStore();
  const configStore = useConfigStore();
  const { game_id } = useParams<{ game_id: string }>();
  const gameId = parseRouteNumericId(game_id);

  const { data: game } = useQuery({
    queryKey: ["admin", "game", gameId, sharedStore.refresh],
    queryFn: async () => {
      const res = await getGame({ id: gameId! });
      return res.game;
    },
    placeholderData: keepPreviousData,
    enabled: gameId != null,
  });

  const options = useMemo(() => {
    return [
      {
        link: `/admin/games/${game_id}`,
        name: t("game:edit.info"),
        icon: <InfoIcon />,
      },
      {
        link: `/admin/games/${game_id}/challenges`,
        name: t("game:edit.challenge"),
        icon: <LibraryIcon />,
      },
      {
        link: `/admin/games/${game_id}/teams`,
        name: t("game:edit.team"),
        icon: <UsersRoundIcon />,
      },
      {
        link: `/admin/games/${game_id}/submissions`,
        name: t("game:edit.submission"),
        icon: <FlagIcon />,
      },
      {
        link: `/admin/games/${game_id}/notices`,
        name: t("game:edit.notice"),
        icon: <MessageCircleIcon />,
      },
    ];
  }, [game_id, t]);

  const [loading, setLoading] = useState<boolean>(false);
  const [updatingState, setUpdatingState] = useState<
    "paused" | "blacked_out" | null
  >(null);
  const [gameState, setGameState] = useState({
    paused: false,
    blacked_out: false,
  });

  useEffect(() => {
    if (!game) return;

    setGameState({
      paused: game.paused,
      blacked_out: game.blacked_out,
    });
  }, [game]);

  async function handleStateChange(
    state: "paused" | "blacked_out",
    checked: boolean
  ) {
    if (gameId == null || updatingState != null) return;

    const previous = gameState[state];
    setGameState((current) => ({ ...current, [state]: checked }));
    setUpdatingState(state);

    try {
      const res = await updateGame({ id: gameId, [state]: checked });
      setGameState({
        paused: res.game.paused,
        blacked_out: res.game.blacked_out,
      });
      toast.success(
        t("game:actions.update.success", { title: res.game.title })
      );
      sharedStore.setRefresh();
    } catch {
      setGameState((current) => ({ ...current, [state]: previous }));
      toast.error(t("common:errors.network"));
    } finally {
      setUpdatingState(null);
    }
  }

  function handleRecalculate() {
    setLoading(true);
    calculateGame({ game_id: gameId! })
      .then(() => {
        toast.success(t("game:edit.recalculate"));
      })
      .finally(() => {
        setLoading(false);
      });
  }

  return (
    <>
      <title>{`${game?.title} - ${configStore?.config?.meta?.title}`}</title>
      <Context.Provider value={{ game }}>
        <div
          className={cn([
            "flex",
            "flex-col",
            "xl:flex-row",
            "xl:min-h-(--app-content-height)",
            "flex-1",
            "min-h-0",
            "xl:pl-64",
          ])}
        >
          <ScrollableNav className={cn(["xl:hidden"])}>
            {options?.map((option, index) => (
              <Button
                key={index}
                icon={option?.icon}
                variant={pathname === option?.link ? "tonal" : "ghost"}
                size="sm"
                className={cn(["shrink-0"])}
                asChild
              >
                <Link to={option?.link}>{option?.name}</Link>
              </Button>
            ))}
          </ScrollableNav>
          <aside
            className={cn([
              "hidden",
              "xl:flex",
              "xl:fixed",
              "xl:left-16",
              "xl:top-16",
              "xl:z-10",
              "xl:h-(--app-content-height)",
              "xl:w-64",
              "xl:flex-col",
              "xl:border-r",
              "xl:bg-card/30",
              "xl:backdrop-blur-sm",
              "py-6",
              "px-4",
              "gap-4",
              "my-6",
              "mx-4",
              "xl:my-0",
              "xl:mx-0",
            ])}
          >
            <div
              className={cn([
                "flex",
                "items-center",
                "gap-2",
                "px-2",
                "text-sm",
                "font-medium",
                "text-muted-foreground",
              ])}
            >
              <FlagIcon className="size-4" />
              {t("game:edit._")}
            </div>
            <nav className={cn(["flex", "flex-col", "gap-1", "flex-1"])}>
              {options?.map((option, index) => (
                <Button
                  key={index}
                  icon={option?.icon}
                  variant={pathname === option?.link ? "tonal" : "ghost"}
                  className={cn(["justify-start"])}
                  asChild
                >
                  <Link to={option?.link}>{option?.name}</Link>
                </Button>
              ))}
            </nav>
            <div
              className={cn(["border-t", "pt-4", "flex", "flex-col", "gap-1"])}
            >
              <div
                className={cn(
                  buttonVariants({ variant: "ghost" }),
                  "justify-start",
                  "w-full",
                  "text-muted-foreground"
                )}
              >
                <CirclePauseIcon className="size-4" />
                <span className="flex-1">{t("game:form.paused._")}</span>
                <Switch
                  checked={gameState.paused}
                  disabled={game == null || updatingState != null}
                  onCheckedChange={(checked) =>
                    handleStateChange("paused", checked)
                  }
                  aria-label={t("game:form.paused._")}
                />
              </div>
              <div
                className={cn(
                  buttonVariants({ variant: "ghost" }),
                  "justify-start",
                  "w-full",
                  "text-muted-foreground"
                )}
              >
                <MoonIcon className="size-4" />
                <span className="flex-1">{t("game:form.blacked_out._")}</span>
                <Switch
                  checked={gameState.blacked_out}
                  disabled={game == null || updatingState != null}
                  onCheckedChange={(checked) =>
                    handleStateChange("blacked_out", checked)
                  }
                  aria-label={t("game:form.blacked_out._")}
                />
              </div>
              <Button
                icon={<RefreshCwIcon className="size-4" />}
                variant="ghost"
                className={cn([
                  "justify-start",
                  "w-full",
                  "text-muted-foreground",
                ])}
                loading={loading}
                onClick={handleRecalculate}
              >
                {t("game:edit.recalculate")}
              </Button>
            </div>
          </aside>
          <Card
            className={cn([
              "flex-1",
              "min-w-0",
              "min-h-0",
              "p-4",
              "sm:p-6",
              "xl:p-10",
              "border-y-0",
              "rounded-none",
              "flex",
              "flex-col",
              "xl:rounded-l-none",
            ])}
          >
            <Outlet />
          </Card>
        </div>
      </Context.Provider>
    </>
  );
}

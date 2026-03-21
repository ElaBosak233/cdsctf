import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  FlagIcon,
  InfoIcon,
  LibraryIcon,
  MessageCircleIcon,
  UsersRoundIcon,
} from "lucide-react";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Link, Outlet, useLocation, useParams } from "react-router";
import { getGames } from "@/api/admin/games";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "./context";

export default function Layout() {
  const { t } = useTranslation();

  const location = useLocation();
  const pathname = location.pathname;
  const sharedStore = useSharedStore();
  const configStore = useConfigStore();
  const { game_id } = useParams<{ game_id: string }>();

  const { data: game } = useQuery({
    queryKey: ["admin", "game", game_id, sharedStore.refresh],
    queryFn: async () => {
      const res = await getGames({
        id: Number(game_id),
      });
      return res?.game;
    },
    placeholderData: keepPreviousData,
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
        link: `/admin/games/${game_id}/notices`,
        name: t("game:edit.notice"),
        icon: <MessageCircleIcon />,
      },
    ];
  }, [game_id, t]);

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
          <nav
            className={cn([
              "xl:hidden",
              "flex",
              "flex-row",
              "flex-wrap",
              "gap-2",
              "p-3",
              "border-b",
              "bg-card/30",
              "shrink-0",
            ])}
          >
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
          </nav>
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
            <nav className={cn(["flex", "flex-col", "gap-1"])}>
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
          </aside>
          <Card
            className={cn([
              "flex-1",
              "min-w-0",
              "min-h-0",
              "p-10",
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

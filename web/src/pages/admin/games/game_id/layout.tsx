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
      return res?.data?.[0];
    },
    placeholderData: keepPreviousData,
  });

  const options = useMemo(() => {
    return [
      {
        link: `/admin/games/${game_id}`,
        name: t("game.edit.info"),
        icon: <InfoIcon />,
      },
      {
        link: `/admin/games/${game_id}/challenges`,
        name: t("game.edit.challenge"),
        icon: <LibraryIcon />,
      },
      {
        link: `/admin/games/${game_id}/teams`,
        name: t("game.edit.team"),
        icon: <UsersRoundIcon />,
      },
      {
        link: `/admin/games/${game_id}/notices`,
        name: t("game.edit.notice"),
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
            "relative",
            "flex",
            "flex-col",
            "xl:flex-row",
            "flex-1",
            "gap-10",
            "xl:mx-30",
          ])}
        >
          <div
            className={cn([
              "space-y-6",
              "h-fit",
              "my-10",
              "mx-10",
              "xl:mx-0",
              "xl:my-0",
              "xl:w-64",
              "xl:sticky",
              "xl:top-24",
            ])}
          >
            <div
              className={cn([
                "flex",
                "flex-wrap",
                "justify-center",
                "gap-3",
                "select-none",
              ])}
            >
              <FlagIcon />
              {t("game.edit._")}
            </div>
            <Card className={cn(["flex", "flex-col", "p-5", "gap-3"])}>
              {options?.map((option, index) => {
                return (
                  <Button
                    key={index}
                    icon={option?.icon}
                    variant={pathname === option?.link ? "tonal" : "ghost"}
                    className={cn(["justify-start"])}
                    asChild
                  >
                    <Link to={option?.link}>{option?.name}</Link>
                  </Button>
                );
              })}
            </Card>
          </div>
          <Card
            className={cn([
              "flex-1",
              "p-10",
              "border-y-0",
              "rounded-none",
              "flex",
              "flex-col",
            ])}
          >
            <Outlet />
          </Card>
        </div>
      </Context.Provider>
    </>
  );
}

import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  ContainerIcon,
  FolderIcon,
  InfoIcon,
  LibraryIcon,
  PencilLineIcon,
  PlayIcon,
  ScrollTextIcon,
} from "lucide-react";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Link, Outlet, useLocation, useParams } from "react-router";
import { getChallenge } from "@/api/admin/challenges/challenge_id";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
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
  const { challenge_id } = useParams<{ challenge_id: string }>();

  const { data: challenge } = useQuery({
    queryKey: ["admin", "challenge", challenge_id, sharedStore.refresh],
    queryFn: async () => {
      const res = await getChallenge({
        id: Number(challenge_id),
      });
      return res.data;
    },
    placeholderData: keepPreviousData,
  });

  const options = useMemo(() => {
    return [
      {
        link: `/admin/challenges/${challenge_id}`,
        name: t("challenge:edit.info"),
        icon: <InfoIcon />,
      },
      {
        link: `/admin/challenges/${challenge_id}/checker`,
        name: t("challenge:edit.checker"),
        icon: <ScrollTextIcon />,
      },
      {
        link: `/admin/challenges/${challenge_id}/attachments`,
        name: t("challenge:edit.attachment"),
        icon: <FolderIcon />,
        disabled: !challenge?.has_attachment,
      },
      {
        link: `/admin/challenges/${challenge_id}/env`,
        name: t("challenge:edit.env"),
        icon: <ContainerIcon />,
        disabled: !challenge?.dynamic,
      },
      {
        link: `/admin/challenges/${challenge_id}/writeup`,
        name: t("challenge:edit.writeup"),
        icon: <PencilLineIcon />,
        disabled: !challenge?.has_writeup,
      },
    ];
  }, [challenge_id, challenge, t]);

  return (
    <>
      <title>{`${challenge?.title} - ${configStore?.config?.meta?.title}`}</title>
      <Context.Provider value={{ challenge }}>
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
            {options?.map((option, index) => {
              const Comp = option?.disabled ? Button : Link;
              return (
                <Button
                  key={index}
                  icon={option?.icon}
                  variant={pathname === option?.link ? "tonal" : "ghost"}
                  size="sm"
                  className={cn(["shrink-0"])}
                  asChild
                  disabled={option?.disabled}
                >
                  <Comp to={option?.link}>{option?.name}</Comp>
                </Button>
              );
            })}
            <Dialog>
              <DialogTrigger>
                <Button variant="ghost" size="sm" className="shrink-0">
                  <PlayIcon className="size-4" />
                  {t("challenge:preview")}
                </Button>
              </DialogTrigger>
              <DialogContent>
                <ChallengeDialog digest={challenge} debug />
              </DialogContent>
            </Dialog>
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
              <LibraryIcon className="size-4" />
              {t("challenge:edit._")}
            </div>
            <nav className={cn(["flex", "flex-col", "gap-1"])}>
              {options?.map((option, index) => {
                const Comp = option?.disabled ? Button : Link;
                return (
                  <Button
                    key={index}
                    icon={option?.icon}
                    variant={pathname === option?.link ? "tonal" : "ghost"}
                    className={cn(["justify-start"])}
                    asChild
                    disabled={option?.disabled}
                  >
                    <Comp to={option?.link}>{option?.name}</Comp>
                  </Button>
                );
              })}
            </nav>
            <div className={cn(["flex-1"])} />
            <Dialog>
              <DialogTrigger>
                <Button variant="ghost" className="justify-start w-full">
                  <PlayIcon className="size-4" />
                  {t("challenge:preview")}
                </Button>
              </DialogTrigger>
              <DialogContent>
                <ChallengeDialog digest={challenge} debug />
              </DialogContent>
            </Dialog>
          </aside>
          <Card
            className={cn([
              "flex-1",
              "min-h-0",
              "min-w-0",
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

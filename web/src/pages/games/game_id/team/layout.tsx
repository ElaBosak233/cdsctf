import {
  CheckCheckIcon,
  CheckIcon,
  FilePenIcon,
  InfoIcon,
  LockIcon,
  TriangleAlertIcon,
  UserRoundMinusIcon,
  UserRoundXIcon,
  UsersRoundIcon,
} from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Link, Outlet, useLocation, useNavigate } from "react-router";
import { toast } from "sonner";
import { deleteTeam, setTeamReady } from "@/api/games/game_id/teams/us";
import { leaveTeam } from "@/api/games/game_id/teams/us/users";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  Dialog,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogHeader,
} from "@/components/ui/dialog";
import { Separator } from "@/components/ui/separator";
import {
  Sidebar,
  SidebarContent,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
} from "@/components/ui/sidebar";
import { State } from "@/models/team";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";
import { getGamePhase, isGameActive } from "@/utils/time";

export default function Layout() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { currentGame, selfTeam, members } = useGameStore();
  const navigate = useNavigate();
  const location = useLocation();
  const pathname = location.pathname;
  const phase = getGamePhase(currentGame ?? {});
  const disabled = phase == null || phase === "ended";
  const isGameOngoing = isGameActive(currentGame ?? {});

  const options = [
    {
      link: `/games/${currentGame?.id}/team`,
      name: t("team:info"),
      icon: <InfoIcon />,
    },
    {
      link: `/games/${currentGame?.id}/team/members`,
      name: t("team:members"),
      icon: <UsersRoundIcon />,
    },
    {
      link: `/games/${currentGame?.id}/team/writeup`,
      name: t("team:write_up._"),
      icon: <FilePenIcon />,
      disabled: !currentGame?.writeup_required || !isGameOngoing,
    },
  ];

  const [confirmDialogOpen, setConfirmDialogOpen] = useState<boolean>(false);

  async function handleSetReady() {
    try {
      await setTeamReady({
        game_id: currentGame?.id,
        id: selfTeam?.id,
      });

      toast.success(t("team:actions.ready.success"), {
        description: t("team:actions.ready.description", {
          name: selfTeam?.name,
        }),
      });
      setConfirmDialogOpen(false);
    } catch (error) {
      await notifyApiError(error);
    }
    sharedStore.setRefresh();
  }

  const [disbandDialogOpen, setDisbandDialogOpen] = useState<boolean>(false);

  async function handleDisband() {
    if (!selfTeam?.id || !currentGame?.id) return;
    try {
      await deleteTeam({
        team_id: selfTeam.id!,
        game_id: currentGame.id!,
      });

      toast.success(t("team:actions.disband.success"), {
        description: t("team:actions.disband.description", {
          name: selfTeam?.name,
        }),
      });
      setDisbandDialogOpen(false);
      navigate(`/games/${currentGame?.id}`);
    } finally {
      sharedStore.setRefresh();
    }
  }

  const [leaveDialogOpen, setLeaveDialogOpen] = useState<boolean>(false);

  async function handleLeave() {
    if (!selfTeam?.id || !currentGame?.id) return;
    try {
      await leaveTeam({
        team_id: selfTeam.id!,
        game_id: currentGame.id!,
      });

      toast.success(t("team:actions.leave.success"), {
        description: t("team:actions.leave.description", {
          name: selfTeam?.name,
        }),
      });
      setLeaveDialogOpen(false);
      navigate(`/games/${currentGame?.id}`);
    } catch (error) {
      await notifyApiError(error, {
        title: t("team:actions.leave.error"),
      });
    } finally {
      sharedStore.setRefresh();
    }
  }

  return (
    <SidebarProvider defaultOpen className="min-h-(--app-content-height)">
      <Sidebar
        collapsible="none"
        className={cn([
          "hidden",
          "lg:w-1/5",
          "bg-card/30",
          "backdrop-blur-sm",
          "lg:flex",
          "flex-col",
          "gap-3",
          "p-5",
          "border-r",
          "lg:sticky",
          "top-16",
        ])}
      >
        <SidebarContent className="overflow-visible">
          <div className={cn(["flex", "flex-1", "flex-col", "gap-3"])}>
            <SidebarMenu>
              {options?.map((option, index) => (
                <SidebarMenuItem key={index}>
                  <SidebarMenuButton
                    isActive={pathname === option.link}
                    disabled={option.disabled}
                    className="h-11 justify-start px-8 text-sm"
                    render={<Link to={option.link} />}
                  >
                    {option.icon}
                    <span>{option.name}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
            <Separator />
            <div className={cn(["flex-1"])} />
            <section className="rounded-lg border border-border/70 bg-card/60 p-3 shadow-xs">
              <div className="flex min-w-0 items-center gap-3">
                <div
                  className={cn(
                    "flex size-9 shrink-0 items-center justify-center rounded-md",
                    selfTeam?.state === State.Preparing
                      ? "bg-success/10 text-success"
                      : "bg-error/10 text-error"
                  )}
                >
                  {selfTeam?.state === State.Preparing ? (
                    <CheckIcon className="size-4" />
                  ) : (
                    <LockIcon className="size-4" />
                  )}
                </div>
                <div className="min-w-0">
                  <p className="text-xs text-muted-foreground">
                    {t("team:state._")}
                  </p>
                  <p className="truncate text-sm font-medium text-foreground">
                    {selfTeam?.state === State.Preparing
                      ? t("team:state.preparing")
                      : t("team:actions.locked")}
                  </p>
                </div>
              </div>
              <Separator className="my-3" />
              <div className="grid grid-cols-2 gap-2">
                <Button
                  size="sm"
                  variant="outline"
                  icon={<UserRoundXIcon />}
                  level="error"
                  className="w-full px-2 text-xs"
                  disabled={selfTeam?.state !== State.Preparing || disabled}
                  onClick={() => setDisbandDialogOpen(true)}
                >
                  {t("team:actions.disband._")}
                </Button>
                <Button
                  size="sm"
                  variant="outline"
                  icon={<UserRoundMinusIcon />}
                  level="warning"
                  className="w-full px-2 text-xs"
                  disabled={
                    selfTeam?.state !== State.Preparing ||
                    members?.length === 1 ||
                    disabled
                  }
                  onClick={() => setLeaveDialogOpen(true)}
                >
                  {t("team:actions.leave._")}
                </Button>
              </div>
              {selfTeam?.state === State.Preparing && (
                <Button
                  size="md"
                  className="mt-2 w-full"
                  icon={<CheckIcon />}
                  level="success"
                  variant="solid"
                  disabled={disabled}
                  onClick={() => setConfirmDialogOpen(true)}
                >
                  {t("team:actions.ready._")}
                </Button>
              )}
            </section>
            <Dialog
              onOpenChange={setDisbandDialogOpen}
              open={disbandDialogOpen}
            >
              <DialogContent>
                <Card
                  className={cn([
                    "w-full",
                    "max-w-xl",
                    "rounded-elevated",
                    "shadow-lg",
                    "overflow-hidden",
                    "flex",
                    "flex-col",
                  ])}
                >
                  <DialogHeader
                    className="p-5 pb-0"
                    icon={<UserRoundXIcon />}
                    level="error"
                    title={t("team:actions.disband._")}
                  />
                  <DialogBody className="px-5 py-5">
                    <p className="text-sm leading-relaxed text-muted-foreground">
                      {t("team:actions.disband.message")}
                    </p>
                  </DialogBody>
                  <DialogFooter className="p-5 pt-0">
                    <Button
                      icon={<CheckCheckIcon />}
                      level="error"
                      variant="solid"
                      onClick={handleDisband}
                    >
                      {t("common:actions.confirm")}
                    </Button>
                  </DialogFooter>
                </Card>
              </DialogContent>
            </Dialog>
            <Dialog onOpenChange={setLeaveDialogOpen} open={leaveDialogOpen}>
              <DialogContent>
                <Card
                  className={cn([
                    "w-full",
                    "max-w-xl",
                    "rounded-elevated",
                    "shadow-lg",
                    "overflow-hidden",
                    "flex",
                    "flex-col",
                  ])}
                >
                  <DialogHeader
                    className="p-5 pb-0"
                    icon={<UserRoundMinusIcon />}
                    level="warning"
                    title={t("team:actions.leave._")}
                  />
                  <DialogBody className="px-5 py-5">
                    <p className="text-sm leading-relaxed text-muted-foreground">
                      {t("team:actions.leave.message")}
                    </p>
                  </DialogBody>
                  <DialogFooter className="p-5 pt-0">
                    <Button
                      icon={<CheckCheckIcon />}
                      level="error"
                      variant="solid"
                      onClick={handleLeave}
                    >
                      {t("common:actions.confirm")}
                    </Button>
                  </DialogFooter>
                </Card>
              </DialogContent>
            </Dialog>
            <Dialog
              onOpenChange={setConfirmDialogOpen}
              open={confirmDialogOpen}
            >
              <DialogContent>
                <Card
                  className={cn([
                    "w-full",
                    "max-w-xl",
                    "rounded-elevated",
                    "shadow-lg",
                    "overflow-hidden",
                    "flex",
                    "flex-col",
                  ])}
                >
                  <DialogHeader
                    className="p-5 pb-0"
                    icon={<TriangleAlertIcon />}
                    level="warning"
                    title={t("team:actions.ready.title")}
                  />
                  <DialogBody className="px-5 py-5">
                    <p className="text-sm leading-relaxed text-muted-foreground">
                      {t("team:actions.ready.message")}
                    </p>
                  </DialogBody>
                  <DialogFooter className="p-5 pt-0">
                    <Button
                      icon={<CheckCheckIcon />}
                      level={"warning"}
                      variant={"solid"}
                      onClick={handleSetReady}
                    >
                      {t("team:actions.ready.of_course")}
                    </Button>
                  </DialogFooter>
                </Card>
              </DialogContent>
            </Dialog>
          </div>
        </SidebarContent>
      </Sidebar>
      <SidebarInset className="min-h-(--app-content-height)">
        <div className={cn(["flex-1", "flex", "flex-col"])}>
          <Outlet />
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}

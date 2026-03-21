import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
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
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Separator } from "@/components/ui/separator";
import { State } from "@/models/team";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { formatApiMsg, parseErrorResponse } from "@/utils/query";

export default function Layout() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { currentGame, selfTeam, members } = useGameStore();
  const navigate = useNavigate();
  const location = useLocation();
  const pathname = location.pathname;
  const disabled = Date.now() / 1000 > Number(currentGame?.ended_at);

  const isGameOngoing =
    Number(currentGame?.started_at) * 1000 < Date.now() &&
    Number(currentGame?.ended_at) * 1000 > Date.now();

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
      if (!(error instanceof HTTPError)) throw error;
      const body = await parseErrorResponse(error);

      if (error.response.status === StatusCodes.BAD_REQUEST) {
        toast.error(t("common:errors.default"), {
          description: formatApiMsg(body.msg),
        });
      }
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
      setDisbandDialogOpen(false);
      navigate(`/games/${currentGame?.id}`);
    } catch (error) {
      if (!(error instanceof HTTPError)) return;
      const body = await parseErrorResponse(error);

      if (error.response.status === StatusCodes.BAD_REQUEST) {
        toast.error(t("team:actions.leave.error"), {
          description: formatApiMsg(body.msg),
        });
      }
    } finally {
      sharedStore.setRefresh();
    }
  }

  return (
    <div className={cn(["flex", "flex-1"])}>
      <div
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
        {options?.map((option, index) => (
          <Button
            key={index}
            size={"lg"}
            className={cn(["justify-start"])}
            icon={option.icon}
            variant={pathname === option.link ? "tonal" : "ghost"}
            disabled={option.disabled}
            asChild={!option.disabled}
          >
            <Link to={option.link}>{option.name}</Link>
          </Button>
        ))}
        <Separator />
        <div className={cn(["flex-1"])} />
        <div className={cn(["flex", "gap-5", "justify-center"])}>
          <Button
            size={"md"}
            icon={<UserRoundXIcon />}
            level={"error"}
            className={cn(["w-1/2"])}
            disabled={selfTeam?.state !== State.Preparing || disabled}
            onClick={() => setDisbandDialogOpen(true)}
          >
            {t("team:actions.disband._")}
          </Button>
          <Dialog onOpenChange={setDisbandDialogOpen} open={disbandDialogOpen}>
            <DialogContent>
              <Card
                className={cn(["flex", "flex-col", "w-lg", "p-5", "gap-5"])}
              >
                <h3
                  className={cn(["flex", "gap-3", "text-md", "items-center"])}
                >
                  <UserRoundXIcon className={cn(["size-4"])} />
                  {t("team:actions.disband._")}
                </h3>
                <p className={cn(["text-sm"])}>
                  {t("team:actions.disband.message")}
                </p>
                <Button
                  icon={<CheckCheckIcon />}
                  level={"error"}
                  variant={"solid"}
                  onClick={handleDisband}
                >
                  {t("common:actions.confirm")}
                </Button>
              </Card>
            </DialogContent>
          </Dialog>
          <Button
            size={"md"}
            icon={<UserRoundMinusIcon />}
            level={"warning"}
            className={cn(["w-1/2"])}
            disabled={
              selfTeam?.state !== State.Preparing ||
              members?.length === 1 ||
              disabled
            }
            onClick={() => setLeaveDialogOpen(true)}
          >
            {t("team:actions.leave._")}
          </Button>
          <Dialog onOpenChange={setLeaveDialogOpen} open={leaveDialogOpen}>
            <DialogContent>
              <Card
                className={cn(["flex", "flex-col", "w-lg", "p-5", "gap-5"])}
              >
                <h3
                  className={cn(["flex", "gap-3", "text-md", "items-center"])}
                >
                  <UserRoundMinusIcon className={cn(["size-4"])} />
                  {t("team:actions.leave._")}
                </h3>
                <p className={cn(["text-sm"])}>
                  {t("team:actions.leave.message")}
                </p>
                <Button
                  icon={<CheckCheckIcon />}
                  level={"error"}
                  variant={"solid"}
                  onClick={handleLeave}
                >
                  {t("common:actions.confirm")}
                </Button>
              </Card>
            </DialogContent>
          </Dialog>
        </div>
        <Button
          size={"lg"}
          className={cn(["justify-start"])}
          icon={
            selfTeam?.state === State.Preparing ? <CheckIcon /> : <LockIcon />
          }
          level={selfTeam?.state === State.Preparing ? "success" : "error"}
          variant={"solid"}
          disabled={selfTeam?.state !== State.Preparing || disabled}
          onClick={() => setConfirmDialogOpen(true)}
        >
          {selfTeam?.state === State.Preparing
            ? t("team:actions.ready._")
            : t("team:actions.locked")}
        </Button>
        <Dialog onOpenChange={setConfirmDialogOpen} open={confirmDialogOpen}>
          <DialogContent>
            <Card className={cn(["flex", "flex-col", "w-lg", "p-5", "gap-5"])}>
              <h3 className={cn(["flex", "gap-3", "text-md", "items-center"])}>
                <TriangleAlertIcon className={cn(["size-4"])} />
                {t("team:actions.ready.title")}
              </h3>
              <p className={cn(["text-sm"])}>
                {t("team:actions.ready.message")}
              </p>
              <Button
                icon={<CheckCheckIcon />}
                level={"warning"}
                variant={"solid"}
                onClick={handleSetReady}
              >
                {t("team:actions.ready.of_course")}
              </Button>
            </Card>
          </DialogContent>
        </Dialog>
      </div>
      <div className={cn(["flex-1", "flex", "flex-col"])}>
        <Outlet />
      </div>
    </div>
  );
}

import {
  ArrowRightIcon,
  CalendarCheckIcon,
  FlagIcon,
  PlayIcon,
  SwordsIcon,
  ThumbsDownIcon,
  UserRoundIcon,
} from "lucide-react";
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Image } from "@/components/ui/image";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { Typography } from "@/components/ui/typography";
import { useTickerTime } from "@/hooks/use-ticker-time";
import { State } from "@/models/team";
import { useAuthStore } from "@/storages/auth";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";
import { TeamGatheringDialog } from "./team-gathering-dialog";

export default function Index() {
  const { t } = useTranslation();

  const { currentGame } = useGameStore();
  const { game_id } = useParams<{ game_id: string }>();

  const status = useMemo(() => {
    if (!currentGame) return "loading";

    const startedAt = new Date(Number(currentGame.started_at) * 1000);
    const endedAt = new Date(Number(currentGame.ended_at) * 1000);

    if (startedAt > new Date()) return "upcoming";
    if (endedAt < new Date()) return "ended";
    return "ongoing";
  }, [currentGame]);

  const now = useTickerTime();

  return (
    <>
      <title>{`${currentGame?.title}`}</title>
      <div
        className={cn([
          "w-full",
          "flex",
          "flex-col",
          "lg:px-10",
          "xl:px-30",
          "lg:flex-row",
          "justify-center",
          "gap-12",
        ])}
      >
        <div
          className={cn([
            "w-full",
            "lg:sticky",
            "lg:top-16",
            "lg:w-2/5",
            "lg:h-[calc(100vh-64px)]",
            "flex",
            "flex-col",
            "lg:justify-between",
            "py-10",
            "px-20",
            "lg:px-0",
            "gap-10",
          ])}
        >
          <div className={cn(["flex", "flex-col", "gap-5", "items-center"])}>
            <Image
              src={currentGame?.has_poster && `/api/games/${game_id}/poster`}
              className={cn([
                "object-cover",
                "rounded-xl",
                "overflow-hidden",
                "border",
                "aspect-video",
                "w-full",
                "bg-card/50",
                "shadow-sm",
                "select-none",
              ])}
              fallback={
                <FlagIcon
                  className={cn([
                    "size-20",
                    "rotate-15",
                    "text-secondary-foreground",
                  ])}
                  strokeWidth={1}
                />
              }
            />
            <h2 className={cn(["text-2xl"])}>{currentGame?.title}</h2>
            {currentGame?.sketch && (
              <p
                className={cn([
                  "max-w-3/4",
                  "text-sm",
                  "text-secondary-foreground",
                  "text-ellipsis",
                  "text-center",
                  "overflow-hidden",
                ])}
              >
                {currentGame?.sketch}
              </p>
            )}
            <Badge
              className={cn(
                ["bg-info", "text-info-foreground", "select-none"],
                status === "ongoing" && [
                  "bg-success",
                  "text-success-foreground",
                ],
                status === "ended" && ["bg-error", "text-error-foreground"],
                status === "upcoming" && ["bg-info", "text-info-foreground"]
              )}
              size={"sm"}
            >
              {new Date(
                Number(currentGame?.started_at) * 1000
              ).toLocaleString()}
              <ArrowRightIcon />
              {new Date(Number(currentGame?.ended_at) * 1000).toLocaleString()}
            </Badge>
            <div
              className={cn([
                "w-full",
                "flex",
                "flex-col",
                "items-center",
                "select-none",
              ])}
            >
              <span className={cn(["text-sm", "text-secondary-foreground"])}>
                {(() => {
                  const startTime = new Date(
                    Number(currentGame?.started_at) * 1000
                  );
                  const freezeTime = new Date(
                    Number(currentGame?.frozen_at) * 1000
                  );
                  const endTime = new Date(
                    Number(currentGame?.ended_at) * 1000
                  );

                  const diff = (target: Date) =>
                    Math.max(
                      0,
                      Math.floor((target.getTime() - now.getTime()) / 1000)
                    );

                  const remain = diff(endTime);
                  const h = Math.floor(remain / 3600);
                  const m = Math.floor((remain % 3600) / 60);
                  const s = remain % 60;

                  if (now < startTime) {
                    return t("game.status.upcoming.remaining", {
                      hours: h,
                      minutes: m,
                      seconds: s,
                    });
                  } else if (now < freezeTime) {
                    return t("game.status.ongoing.remaining", {
                      hours: h,
                      minutes: m,
                      seconds: s,
                    });
                  } else if (now < endTime) {
                    return t("game.status.frozen.remaining", {
                      hours: h,
                      minutes: m,
                      seconds: s,
                    });
                  } else {
                    return t("game.status.ended.remaining");
                  }
                })()}
              </span>
            </div>
          </div>
          <div>
            <GameActionButton
              status={status as "ongoing" | "ended" | "upcoming"}
            />
          </div>
        </div>
        <Card
          className={cn([
            "lg:w-3/5",
            "min-h-[calc(100vh-64px)]",
            "p-15",
            "rounded-none",
            "border-y-0",
            "shadow-md",
            "relative",
          ])}
        >
          <Typography>
            <LoadingOverlay loading={!currentGame} />
            {currentGame &&
              (currentGame?.description ? (
                <MarkdownRender src={currentGame?.description} />
              ) : (
                <div
                  className={cn([
                    "absolute",
                    "inset-0",
                    "flex",
                    "flex-col",
                    "justify-center",
                    "items-center",
                    "gap-5",
                    "select-none",
                  ])}
                >
                  <ThumbsDownIcon className={cn(["size-12"])} />
                  {t("game.description.empty")}
                </div>
              ))}
          </Typography>
        </Card>
      </div>
    </>
  );
}

interface GameActionProps {
  status: "ongoing" | "ended" | "upcoming";
}

export function GameActionButton({ status }: GameActionProps) {
  const { t } = useTranslation();

  const { user } = useAuthStore();
  const { selfTeam } = useGameStore();
  const navigate = useNavigate();
  const { game_id } = useParams<{ game_id: string }>();

  const [teamGatheringDialogOpen, setTeamGatheringDialogOpen] = useState(false);

  const invalidMessage = useMemo(() => {
    if (selfTeam?.state === State.Banned) {
      return t("team.states.banned");
    } else if (selfTeam?.state === State.Preparing) {
      return t("team.states.preparing");
    } else if (selfTeam?.state === State.Pending) {
      return t("team.states.pending");
    }
    return undefined;
  }, [selfTeam, t]);

  /** --- 比赛已结束 --- */
  if (status === "ended") {
    return (
      <Button
        className="w-full"
        variant="solid"
        level="error"
        size="lg"
        icon={<CalendarCheckIcon />}
        disabled
      >
        {t("game.status.ended.remaining")}
      </Button>
    );
  }

  /** --- 未登录用户 --- */
  if (!user?.id) {
    return (
      <Button
        className="w-full"
        variant="solid"
        level="warning"
        size="lg"
        icon={<UserRoundIcon />}
        disabled
      >
        {t("team.actions.participate_after_login")}
      </Button>
    );
  }

  /** --- 已登录但无队伍 --- */
  if (!selfTeam) {
    return (
      <>
        <Button
          className="w-full"
          variant="solid"
          level="info"
          size="lg"
          icon={<SwordsIcon />}
          onClick={() => setTeamGatheringDialogOpen(true)}
        >
          {t("team.actions.gather._")}
        </Button>

        <Dialog
          open={teamGatheringDialogOpen}
          onOpenChange={setTeamGatheringDialogOpen}
        >
          <DialogContent>
            <TeamGatheringDialog
              onClose={() => setTeamGatheringDialogOpen(false)}
            />
          </DialogContent>
        </Dialog>
      </>
    );
  }

  /** --- 已登录且有队伍 --- */
  const isOngoing = status === "ongoing";
  const canParticipate = isOngoing && selfTeam.state === State.Passed;

  return (
    <Button
      className="w-full"
      variant="solid"
      level="success"
      size="lg"
      icon={<PlayIcon />}
      disabled={!canParticipate}
      onClick={() => navigate(`/games/${game_id}/challenges`)}
    >
      <span>{t("team.actions.participate", { name: selfTeam.name })}</span>
      {invalidMessage && <span>（{invalidMessage}）</span>}
    </Button>
  );
}

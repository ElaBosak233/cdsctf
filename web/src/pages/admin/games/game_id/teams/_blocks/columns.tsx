import {
  BanIcon,
  CheckCheckIcon,
  FileCheck2Icon,
  FlagIcon,
  Undo2Icon,
} from "lucide-react";
import { useContext } from "react";
import { useTranslation } from "react-i18next";
import { Link, useParams } from "react-router";
import { toast } from "sonner";
import { updateTeam } from "@/api/management/games/game_id/teams/team_id";
import { Avatar, AvatarGroup, AvatarGroupCount } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { ColumnDef } from "@/hooks/use-data-table";
import { State, type TeamView } from "@/models/team";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { notifyApiError, parseRouteNumericId } from "@/utils/query";
import { Context } from "../../context";

function useColumns(): Array<ColumnDef<TeamView>> {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);
  const resolvedGameId = routeGameId ?? game?.id;

  return [
    {
      accessorKey: "id",
      id: "id",
      header: t("team:id"),
      enableHiding: true,
    },
    {
      accessorKey: "name",
      id: "name",
      header: t("team:name"),
      cell: ({ row }) => {
        const name = row.original.name!;
        return (
          <div className={cn(["flex", "min-w-0", "items-center", "gap-3"])}>
            <Avatar
              className={cn(["size-9", "shrink-0"])}
              src={
                row.original.avatar_hash &&
                `/api/media?hash=${row.original.avatar_hash}`
              }
              fallback={name.charAt(0)}
            />
            <div className={cn(["min-w-0", "flex-1"])}>
              <div className={cn(["truncate", "text-sm", "font-semibold"])}>
                {name}
              </div>
              <div
                className={cn([
                  "mt-0.5",
                  "flex",
                  "min-w-0",
                  "items-center",
                  "gap-1.5",
                  "text-xs",
                  "text-muted-foreground",
                ])}
              >
                <span className={cn(["shrink-0", "font-mono"])}>
                  #{row.original.id}
                </span>
                {row.original.slogan && (
                  <>
                    <span className="shrink-0">·</span>
                    <span className="truncate">{row.original.slogan}</span>
                  </>
                )}
              </div>
            </div>
          </div>
        );
      },
    },
    {
      id: "members",
      header: t("team:members"),
      cell: ({ row }) => {
        const members = row.original.members ?? [];

        if (members.length === 0) {
          return <span className="text-muted-foreground">/</span>;
        }

        return (
          <div className="flex min-w-36 items-center pl-1">
            <AvatarGroup>
              {members.slice(0, 10).map((member, index) => (
                <Tooltip key={member.id}>
                  <TooltipTrigger
                    className={cn(["rounded-full", index > 0 && "-ml-2"])}
                  >
                    <Avatar
                      className="size-9"
                      src={
                        member.avatar_hash &&
                        `/api/media?hash=${member.avatar_hash}`
                      }
                      fallback={member.name.charAt(0)}
                    />
                  </TooltipTrigger>
                  <TooltipContent>
                    {member.name || member.username}
                  </TooltipContent>
                </Tooltip>
              ))}
              {members.length > 10 && (
                <AvatarGroupCount>+{members.length - 10}</AvatarGroupCount>
              )}
            </AvatarGroup>
          </div>
        );
      },
    },
    {
      accessorKey: "rank",
      id: "rank",
      header: () => <div className="text-right">{t("team:rank")}</div>,
      cell: ({ row }) => {
        const value = row.original.rank;
        const banned = row.original.state === State.Banned;

        return (
          <div className="flex justify-end">
            {value === 0 ? (
              <span
                aria-hidden="true"
                className="inline-flex h-6 w-10 select-none items-center justify-center rounded-sm border border-border/40 bg-muted/30 font-mono text-xs text-muted-foreground shadow-inner backdrop-blur-sm"
              >
                /
              </span>
            ) : (
              <span
                className={cn([
                  "font-mono",
                  "text-xs",
                  "tabular-nums",
                  banned &&
                    "select-none text-muted-foreground line-through decoration-error/70",
                ])}
              >
                {value}
              </span>
            )}
          </div>
        );
      },
    },
    {
      accessorKey: "pts",
      id: "pts",
      header: () => <div className="text-right">{t("team:pts")}</div>,
      cell: ({ row }) => {
        const value = row.original.pts;
        const banned = row.original.state === State.Banned;

        return (
          <div className="flex justify-end">
            {value === 0 ? (
              <span
                aria-hidden="true"
                className="inline-flex h-6 w-10 select-none items-center justify-center rounded-sm border border-border/40 bg-muted/30 font-mono text-xs text-muted-foreground shadow-inner backdrop-blur-sm"
              >
                /
              </span>
            ) : (
              <span
                className={cn([
                  "font-mono",
                  "text-xs",
                  "tabular-nums",
                  banned &&
                    "select-none text-muted-foreground line-through decoration-error/70",
                ])}
              >
                {value}
              </span>
            )}
          </div>
        );
      },
    },
    {
      accessorKey: "state",
      id: "state",
      header: t("team:state._"),
      cell: ({ row }) => {
        const state = row.original.state;

        switch (state) {
          case State.Banned:
            return (
              <Badge className={cn(["bg-error", "text-error-foreground"])}>
                {t("team:state.banned")}
              </Badge>
            );
          case State.Preparing:
            return (
              <Badge className={cn(["bg-info", "text-info-foreground"])}>
                {t("team:state.preparing")}
              </Badge>
            );
          case State.Pending:
            return (
              <Badge className={cn(["bg-warning", "text-warning-foreground"])}>
                {t("team:state.pending")}
              </Badge>
            );
          case State.Passed:
            return (
              <Badge className={cn(["bg-success", "text-success-foreground"])}>
                {t("team:state.passed")}
              </Badge>
            );
        }
      },
    },
    ...(game?.writeup_required
      ? [
          {
            id: "has_writeup",
            header: t("team:has_writeup._"),
            cell: function WriteUpCell({ row }) {
              const has_writeup = row.original.has_writeup;

              return (
                <div className={cn(["flex", "items-center", "gap-2"])}>
                  {has_writeup ? (
                    <Badge className={cn(["bg-info", "text-info-foreground"])}>
                      {t("team:has_writeup.true")}
                    </Badge>
                  ) : (
                    <Badge
                      className={cn(["bg-warning", "text-warning-foreground"])}
                    >
                      {t("team:has_writeup.false")}
                    </Badge>
                  )}
                  <Button
                    size={"sm"}
                    variant={"ghost"}
                    square
                    disabled={!has_writeup || resolvedGameId == null}
                    render={
                      has_writeup ? (
                        <Link
                          to={
                            resolvedGameId != null
                              ? `/api/management/games/${resolvedGameId}/teams/${row.original.id}/writeup`
                              : "#"
                          }
                          target={"_blank"}
                        />
                      ) : undefined
                    }
                  >
                    <FileCheck2Icon />
                  </Button>
                </div>
              );
            },
          } satisfies ColumnDef<TeamView>,
        ]
      : []),
    {
      id: "actions",
      header: () => (
        <div className={cn(["justify-self-center"])}>
          {t("game:team.actions._")}
        </div>
      ),
      cell: function ActionsCell({ row }) {
        const id = row.original.id;
        const game_id = row.original.game_id;
        const resolvedTeamGameId = game_id ?? resolvedGameId;
        const state = row.original.state;

        async function handleStateChange(state: State) {
          const gid = game_id ?? resolvedGameId;
          if (gid == null || id == null) return;

          try {
            await updateTeam({ team_id: id, game_id: gid, state });
            toast.success(
              t("game:team.actions.message", { name: row.original.name })
            );
          } catch (error) {
            await notifyApiError(error);
          } finally {
            sharedStore?.setRefresh();
          }
        }

        return (
          <div className="flex items-center justify-end gap-1.5">
            <Tooltip>
              <TooltipTrigger
                render={
                  <Button
                    disabled={resolvedTeamGameId == null}
                    variant={"ghost"}
                    size={"sm"}
                    square
                    className="text-muted-foreground hover:text-foreground"
                    icon={<FlagIcon />}
                    aria-label={t("submission:_")}
                    render={
                      resolvedTeamGameId != null ? (
                        <Link
                          to={`/admin/games/${resolvedTeamGameId}/submissions?team_id=${id}`}
                        />
                      ) : undefined
                    }
                  />
                }
              />
              <TooltipContent>{t("submission:_")}</TooltipContent>
            </Tooltip>
            <Separator orientation={"vertical"} className={"h-6"} />
            <Tooltip>
              <TooltipTrigger
                render={
                  <Button
                    disabled={state === State.Preparing}
                    variant="ghost"
                    size="sm"
                    level="warning"
                    square
                    icon={<Undo2Icon />}
                    aria-label={t("game:team.actions.refuse")}
                    onClick={() => handleStateChange(State.Preparing)}
                  />
                }
              />
              <TooltipContent>{t("game:team.actions.refuse")}</TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger
                render={
                  <Button
                    disabled={state === State.Banned}
                    variant="ghost"
                    size="sm"
                    level="error"
                    square
                    icon={<BanIcon />}
                    aria-label={t("game:team.actions.ban")}
                    onClick={() => handleStateChange(State.Banned)}
                  />
                }
              />
              <TooltipContent>{t("game:team.actions.ban")}</TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger
                render={
                  <Button
                    disabled={state === State.Passed}
                    variant="ghost"
                    size="sm"
                    level="success"
                    square
                    icon={<CheckCheckIcon />}
                    aria-label={t("game:team.actions.pass")}
                    onClick={() => handleStateChange(State.Passed)}
                  />
                }
              />
              <TooltipContent>{t("game:team.actions.pass")}</TooltipContent>
            </Tooltip>
          </div>
        );
      },
    },
  ];
}

export { useColumns };

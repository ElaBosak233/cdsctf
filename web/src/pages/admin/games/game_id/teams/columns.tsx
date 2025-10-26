import type { ColumnDef } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import {
  BanIcon,
  CheckCheckIcon,
  ChevronDownIcon,
  ChevronUpIcon,
  FileCheck2Icon,
  Undo2Icon,
} from "lucide-react";
import { useContext } from "react";
import { Link } from "react-router";
import { toast } from "sonner";
import { updateTeam } from "@/api/admin/games/game_id/teams/team_id";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { State, type Team } from "@/models/team";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";

function useColumns(): Array<ColumnDef<Team>> {
  const sharedStore = useSharedStore();
  const { game } = useContext(Context);

  return [
    {
      accessorKey: "id",
      id: "id",
      header: "ID",
      cell: function IdCell({ row }) {
        const id = row.original.id;
        return (
          <div className={cn(["flex", "items-center", "gap-2"])}>
            <Badge># {id}</Badge>
          </div>
        );
      },
    },
    {
      accessorKey: "name",
      id: "name",
      header: "团队名",
      cell: ({ row }) => {
        const name = row.original.name!;
        return (
          <div className={cn(["flex", "gap-2", "items-center"])}>
            <Avatar
              src={
                row.original.has_avatar &&
                `/api/games/${row.original.game_id}/teams/${row.original?.id}/avatar`
              }
              fallback={name.charAt(0)}
            />
            <span>{name}</span>
          </div>
        );
      },
    },
    {
      accessorKey: "rank",
      id: "rank",
      header: "排名",
      cell: ({ row }) => row.original.rank,
    },
    {
      accessorKey: "pts",
      id: "pts",
      header: "得分",
      cell: ({ row }) => row.original.pts,
    },
    {
      accessorKey: "state",
      id: "state",
      header: "当前状态",
      cell: ({ row }) => {
        const state = row.original.state;

        switch (state) {
          case State.Banned:
            return (
              <Badge className={cn(["bg-error", "text-error-foreground"])}>
                禁赛中
              </Badge>
            );
          case State.Preparing:
            return (
              <Badge className={cn(["bg-info", "text-info-foreground"])}>
                准备中
              </Badge>
            );
          case State.Pending:
            return (
              <Badge className={cn(["bg-warning", "text-warning-foreground"])}>
                待审核
              </Badge>
            );
          case State.Passed:
            return (
              <Badge className={cn(["bg-success", "text-success-foreground"])}>
                正常参赛
              </Badge>
            );
        }
      },
    },
    ...(game?.is_need_write_up
      ? [
          {
            id: "has_write_up",
            header: "Write-up",
            cell: function WriteUpCell({ row }) {
              const has_write_up = row.original.has_write_up;

              return (
                <div className={cn(["flex", "items-center", "gap-2"])}>
                  {has_write_up ? (
                    <Badge className={cn(["bg-info", "text-info-foreground"])}>
                      已提交
                    </Badge>
                  ) : (
                    <Badge
                      className={cn(["bg-warning", "text-warning-foreground"])}
                    >
                      未提交
                    </Badge>
                  )}
                  <Button
                    asChild={has_write_up}
                    size={"sm"}
                    variant={"ghost"}
                    square
                    disabled={!has_write_up}
                  >
                    <Link
                      to={`/api/admin/games/${game.id}/teams/${row.original.id}/writeup`}
                      target={"_blank"}
                    >
                      <FileCheck2Icon />
                    </Link>
                  </Button>
                </div>
              );
            },
          } satisfies ColumnDef<Team>,
        ]
      : []),
    {
      id: "actions",
      header: () => <div className={cn(["justify-self-center"])}>操作</div>,
      cell: function ActionsCell({ row }) {
        const id = row.original.id;
        const game_id = row.original.game_id;
        const state = row.original.state;

        function handleStateChange(state: State) {
          updateTeam({
            team_id: id!,
            game_id: game_id!,
            state,
          })
            .then((res) => {
              if (res.code === StatusCodes.OK) {
                toast.success(`团队 ${row.original.name} 状态更新成功`);
              }
            })
            .finally(() => {
              sharedStore?.setRefresh();
            });
        }

        return (
          <div
            className={cn(["flex", "items-center", "justify-center", "gap-2"])}
          >
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  disabled={state === State.Preparing}
                  variant={"ghost"}
                  size={"sm"}
                  level={"info"}
                  square
                  icon={<Undo2Icon />}
                  onClick={() => handleStateChange(State.Preparing)}
                />
              </TooltipTrigger>
              <TooltipContent>打回</TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  disabled={state === State.Banned}
                  variant={"ghost"}
                  size={"sm"}
                  level={"error"}
                  square
                  icon={<BanIcon />}
                  onClick={() => handleStateChange(State.Banned)}
                />
              </TooltipTrigger>
              <TooltipContent>禁赛</TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  disabled={state === State.Passed}
                  variant={"ghost"}
                  size={"sm"}
                  level={"success"}
                  square
                  icon={<CheckCheckIcon />}
                  onClick={() => handleStateChange(State.Passed)}
                />
              </TooltipTrigger>
              <TooltipContent>通过审核</TooltipContent>
            </Tooltip>
          </div>
        );
      },
    },
    {
      id: "expand",
      cell: ({ row }) => (
        <div className="flex justify-end">
          <Button
            onClick={() => row.toggleExpanded()}
            icon={row.getIsExpanded() ? <ChevronUpIcon /> : <ChevronDownIcon />}
            square
            size={"sm"}
          />
        </div>
      ),
    },
  ];
}

export { useColumns };

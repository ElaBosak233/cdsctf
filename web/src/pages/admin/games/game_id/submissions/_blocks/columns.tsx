import type { ColumnDef } from "@tanstack/react-table";
import { BanIcon, XIcon } from "lucide-react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { updateSubmissionStatus } from "@/api/admin/submissions";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Status, type Submission } from "@/models/submission";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

function useColumns(): Array<ColumnDef<Submission>> {
  const { t } = useTranslation();
  const sharedStore = useSharedStore();

  return [
    {
      accessorKey: "id",
      id: "id",
      header: t("submission:id"),
      cell: ({ row }) => {
        const id = row.original.id;
        return (
          <div className={cn(["flex", "items-center", "gap-2"])}>
            <Badge># {id}</Badge>
          </div>
        );
      },
    },
    {
      accessorKey: "team_name",
      id: "team_name",
      header: t("submission:team_name"),
      cell: ({ row }) => {
        const name = row.original.team_name ?? "-";
        return (
          <div className={cn(["flex", "gap-2", "items-center"])}>
            <Avatar
              src={
                row.original.team_avatar_hash &&
                `/api/media?hash=${row.original.team_avatar_hash}`
              }
              fallback={name.charAt(0)}
            />
            <span>{name}</span>
          </div>
        );
      },
    },
    {
      accessorKey: "challenge_title",
      id: "challenge_title",
      header: t("submission:challenge_title"),
      cell: ({ row }) => <span>{row.original.challenge_title ?? "-"}</span>,
    },
    {
      accessorKey: "team_id",
      id: "team_id",
      header: t("submission:team_id"),
      enableHiding: true,
    },
    {
      accessorKey: "challenge_id",
      id: "challenge_id",
      header: t("submission:challenge_id"),
      enableHiding: true,
    },
    {
      accessorKey: "content",
      id: "content",
      header: t("submission:flag"),
      cell: ({ row }) => {
        const content = row.original.content ?? "";
        return (
          <div
            className={cn([
              "font-mono",
              "truncate",
              "max-w-64",
              "text-muted-foreground",
              "text-xs",
            ])}
          >
            {content}
          </div>
        );
      },
    },
    {
      accessorKey: "status",
      id: "status",
      header: t("submission:status._"),
      cell: ({ row }) => {
        const status = row.original.status;

        switch (status) {
          case Status.Pending:
            return (
              <Badge className={cn(["bg-warning", "text-warning-foreground"])}>
                {t("submission:status.pending")}
              </Badge>
            );
          case Status.Correct:
            return (
              <Badge className={cn(["bg-success", "text-success-foreground"])}>
                {t("submission:status.correct")}
              </Badge>
            );
          case Status.Incorrect:
            return <Badge>{t("submission:status.incorrect")}</Badge>;
          case Status.Cheat:
            return (
              <Badge className={cn(["bg-error", "text-error-foreground"])}>
                {t("submission:status.cheat")}
              </Badge>
            );
          case Status.Expired:
            return (
              <Badge className={cn(["bg-muted", "text-muted-foreground"])}>
                {t("submission:status.expired")}
              </Badge>
            );
          case Status.Duplicate:
            return (
              <Badge className={cn(["bg-info", "text-info-foreground"])}>
                {t("submission:status.duplicate")}
              </Badge>
            );
        }
      },
    },
    {
      accessorKey: "user_name",
      id: "user_name",
      header: t("submission:user_name"),
      cell: ({ row }) => {
        const name = row.original.user_name ?? "-";
        return (
          <div className={cn(["flex", "gap-2", "items-center"])}>
            <Avatar
              src={
                row.original.user_avatar_hash &&
                `/api/media?hash=${row.original.user_avatar_hash}`
              }
              fallback={name.charAt(0)}
            />
            <span>{name}</span>
          </div>
        );
      },
    },
    {
      accessorKey: "created_at",
      id: "created_at",
      header: t("submission:created_at"),
      cell: ({ row }) => {
        const ts = row.original.created_at;
        if (ts == null) return "-";
        return (
          <span className={cn(["text-xs", "text-muted-foreground"])}>
            {new Date(Number(ts) * 1000).toLocaleString()}
          </span>
        );
      },
    },
    {
      id: "actions",
      cell: function ActionsCell({ row }) {
        const id = row.original.id;

        function handleStatusChange(status: Status) {
          if (id == null) return;

          updateSubmissionStatus({
            submission_id: id,
            status,
          })
            .then(() => {
              toast.success(t("submission:actions.status_updated"));
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
                  variant={"ghost"}
                  size={"sm"}
                  level={"error"}
                  square
                  icon={<BanIcon />}
                  onClick={() => handleStatusChange(Status.Cheat)}
                />
              </TooltipTrigger>
              <TooltipContent>
                {t("submission:actions.mark_cheat")}
              </TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant={"ghost"}
                  size={"sm"}
                  square
                  icon={<XIcon />}
                  onClick={() => handleStatusChange(Status.Incorrect)}
                />
              </TooltipTrigger>
              <TooltipContent>
                {t("submission:actions.mark_incorrect")}
              </TooltipContent>
            </Tooltip>
          </div>
        );
      },
    },
  ];
}

export { useColumns };

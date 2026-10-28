import {
  BanIcon,
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  XIcon,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { updateSubmissionStatus } from "@/api/management/submissions";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import type { ColumnDef, Row } from "@/hooks/use-data-table";
import { Status, type SubmissionView } from "@/models/submission";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { notifyApiError } from "@/utils/query";
import { formatDuration, millisecondsBetween } from "@/utils/time";

function ChallengeCell({ row }: { row: Row<SubmissionView> }) {
  const challenge = row.original;
  const id = challenge.challenge_id;
  const category = getCategory(challenge.challenge_category);
  const CategoryIcon = category.icon!;
  const { t } = useTranslation();
  const { isCopied, copyToClipboard } = useClipboard();

  return (
    <div className={cn(["flex", "min-w-0", "items-center", "gap-3"])}>
      <div
        className={cn([
          "flex",
          "size-9",
          "shrink-0",
          "items-center",
          "justify-center",
          "rounded-md",
        ])}
        style={{
          backgroundColor: `${category.color}1a`,
          color: category.color,
        }}
      >
        <CategoryIcon className="size-4" />
      </div>
      <div className={cn(["min-w-0", "flex-1"])}>
        <div className={cn(["truncate", "text-sm", "font-semibold"])}>
          {challenge.challenge_title || "-"}
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
          <span className={cn(["shrink-0", "font-mono"])}>#{id}</span>
          <Tooltip>
            <TooltipTrigger
              render={
                <Button
                  icon={
                    isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />
                  }
                  square
                  size="sm"
                  variant="ghost"
                  className={cn([
                    "size-6",
                    "shrink-0",
                    "text-muted-foreground",
                  ])}
                  aria-label={t("common:tooltip.copy")}
                  onClick={() => copyToClipboard(String(id))}
                />
              }
            />
            <TooltipContent>{t("common:tooltip.copy")}</TooltipContent>
          </Tooltip>
        </div>
      </div>
    </div>
  );
}

function useColumns(): Array<ColumnDef<SubmissionView>> {
  const { t } = useTranslation();
  const sharedStore = useSharedStore();

  return [
    {
      accessorKey: "id",
      id: "id",
      header: t("submission:id"),
      cell: ({ row }) => {
        const id = row.original.id;
        return <span className={cn(["shrink-0", "font-mono"])}>#{id}</span>;
      },
    },
    {
      accessorKey: "team_name",
      id: "team_name",
      header: t("submission:team_name"),
      cell: ({ row }) => {
        const name = row.original.team_name ?? "-";
        return (
          <div className={cn(["flex", "min-w-0", "items-center", "gap-3"])}>
            <Avatar
              className="size-8 shrink-0"
              src={
                row.original.team_avatar_hash &&
                `/api/media?hash=${row.original.team_avatar_hash}`
              }
              fallback={name.charAt(0)}
            />
            <div className="flex min-w-0 flex-col gap-0.5">
              <span className="truncate text-sm font-medium w-36">{name}</span>
              <span className="font-mono text-xs text-muted-foreground">
                #{row.original.team_id}
              </span>
            </div>
          </div>
        );
      },
    },
    {
      accessorKey: "challenge_title",
      id: "challenge_title",
      header: t("submission:challenge_title"),
      cell: ChallengeCell,
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
      accessorKey: "user_id",
      id: "user_id",
      header: t("user:id"),
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
              "max-w-lg",
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
          case Status.Queued:
            return (
              <Badge className={cn(["bg-warning", "text-warning-foreground"])}>
                {t("submission:status.queued")}
              </Badge>
            );
          case Status.Processing:
            return (
              <Badge className={cn(["bg-info", "text-info-foreground"])}>
                {t("submission:status.processing")}
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
          case Status.Failed:
            return (
              <Badge className={cn(["bg-error", "text-error-foreground"])}>
                {t("submission:status.failed")}
              </Badge>
            );
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
      id: "score_rank",
      header: t("submission:score_rank"),
      cell: ({ row }) => {
        if (row.original.status !== Status.Correct) {
          return (
            <span
              aria-hidden="true"
              className="inline-flex h-6 w-10 select-none items-center justify-center rounded-sm border border-border/40 bg-muted/30 font-mono text-xs text-muted-foreground shadow-inner backdrop-blur-sm"
            >
              /
            </span>
          );
        }

        return (
          <div className="flex items-center gap-2 whitespace-nowrap font-mono text-xs tabular-nums">
            <span className="font-semibold text-success">
              +{row.original.pts}
            </span>
            <span className="text-muted-foreground">#{row.original.rank}</span>
          </div>
        );
      },
    },
    {
      accessorKey: "user_name",
      id: "user_name",
      header: t("submission:user_name"),
      cell: ({ row }) => {
        const name = row.original.user_name ?? "-";
        return (
          <div className={cn(["flex", "min-w-0", "items-center", "gap-3"])}>
            <Avatar
              className="size-8 shrink-0"
              src={
                row.original.user_avatar_hash &&
                `/api/media?hash=${row.original.user_avatar_hash}`
              }
              fallback={name.charAt(0)}
            />
            <div className="flex min-w-0 flex-col gap-0.5">
              <span className="truncate text-sm font-medium w-36">{name}</span>
              <span className="font-mono text-xs text-muted-foreground">
                #{row.original.user_id}
              </span>
            </div>
          </div>
        );
      },
    },
    {
      accessorKey: "processing_at",
      id: "processing_duration",
      header: t("submission:processing_duration"),
      cell: ({ row }) => {
        const processingAt = row.original.processing_at;
        const checkedAt = row.original.checked_at;
        if (processingAt == null || checkedAt == null) return "/";

        const duration = millisecondsBetween(processingAt, checkedAt);
        return (
          <span
            className={cn([
              "font-mono",
              "text-xs",
              "tabular-nums",
              "text-muted-foreground",
            ])}
          >
            {formatDuration(duration)}
          </span>
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
            {new Date(ts).toLocaleString()}
          </span>
        );
      },
    },
    {
      id: "actions",
      header: t("submission:actions:_"),
      cell: function ActionsCell({ row }) {
        const id = row.original.id;

        async function handleStatusChange(status: Status) {
          if (id == null) return;

          try {
            await updateSubmissionStatus({ submission_id: id, status });
            toast.success(t("submission:actions.status_updated"));
          } catch (error) {
            await notifyApiError(error);
          } finally {
            sharedStore?.setRefresh();
          }
        }

        return (
          <div
            className={cn([
              "flex",
              "items-center",
              "justify-center",
              "gap-2",
              "*:shrink-0",
            ])}
          >
            <Tooltip>
              <TooltipTrigger
                render={
                  <Button
                    variant={"ghost"}
                    size={"sm"}
                    level={"error"}
                    square
                    icon={<BanIcon />}
                    onClick={() => handleStatusChange(Status.Cheat)}
                  />
                }
              ></TooltipTrigger>
              <TooltipContent>
                {t("submission:actions.mark_cheat")}
              </TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger
                render={
                  <Button
                    variant={"ghost"}
                    size={"sm"}
                    square
                    icon={<XIcon />}
                    onClick={() => handleStatusChange(Status.Incorrect)}
                  />
                }
              ></TooltipTrigger>
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

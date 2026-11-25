import { useQueryClient } from "@tanstack/react-query";
import {
  CalendarPlusIcon,
  Clock3Icon,
  MegaphoneIcon,
  PencilIcon,
  TrashIcon,
} from "lucide-react";
import { type CSSProperties, useContext, useMemo, useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { toast } from "sonner";
import { deleteGameNotice } from "@/api/management/games/game_id/notices";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  Dialog,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogHeader,
} from "@/components/ui/dialog";
import { ContentDialog } from "@/components/widgets/content-dialog";
import type { ColumnDef, Row } from "@/hooks/use-data-table";
import type { AdminGameNoticeView } from "@/models/game_notice";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { notifyApiError, parseRouteNumericId } from "@/utils/query";
import { parseTimestamp } from "@/utils/time";
import { Context } from "../../context";
import { NoticeFormDialog } from "./notice-form-dialog";

function ActionsCell({ row }: { row: Row<AdminGameNoticeView> }) {
  const { t } = useTranslation();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);
  const sharedStore = useSharedStore();
  const queryClient = useQueryClient();

  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);

  async function handleDelete() {
    const gid = routeGameId ?? game?.id ?? row.original.game_id;
    if (gid == null || row.original.id == null) return;

    try {
      await deleteGameNotice({ game_id: gid, id: row.original.id });
      toast.success(
        t("game:notice.actions.delete.success", {
          title: row.original.title,
        })
      );
      setDeleteDialogOpen(false);
      await queryClient.invalidateQueries({
        queryKey: ["admin", "game-notices", gid],
      });
      await queryClient.invalidateQueries({
        queryKey: ["game_notices", gid],
      });
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
        "gap-1.5",
        "[&>*]:shrink-0",
      ])}
    >
      <Button
        variant="ghost"
        size="sm"
        square
        icon={<PencilIcon />}
        aria-label={t("game:notice.actions.edit._")}
        onClick={() => setEditDialogOpen(true)}
      />
      <Dialog open={editDialogOpen} onOpenChange={setEditDialogOpen}>
        <DialogContent size="wide">
          {editDialogOpen && (
            <NoticeFormDialog
              notice={row.original}
              onClose={() => setEditDialogOpen(false)}
            />
          )}
        </DialogContent>
      </Dialog>

      <Button
        level="error"
        variant="ghost"
        size="sm"
        square
        icon={<TrashIcon />}
        aria-label={t("game:notice.actions.delete._")}
        onClick={() => setDeleteDialogOpen(true)}
      />
      <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
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
              icon={<TrashIcon />}
              level="error"
              title={t("game:notice.actions.delete._")}
            />
            <DialogBody className="px-5 py-5">
              <p className="text-sm leading-relaxed text-muted-foreground">
                <Trans
                  i18nKey="game:notice.actions.delete.message"
                  values={{ title: row.original.title }}
                  components={{
                    muted: <span className={cn(["text-muted-foreground"])} />,
                  }}
                />
              </p>
            </DialogBody>
            <DialogFooter className="p-5 pt-0">
              <Button
                level="error"
                variant="tonal"
                size="sm"
                onClick={handleDelete}
              >
                {t("common:actions.confirm")}
              </Button>
            </DialogFooter>
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function useColumns() {
  const { t, i18n } = useTranslation();
  const language = i18n.resolvedLanguage ?? i18n.language;
  const formatter = useMemo(
    () =>
      new Intl.DateTimeFormat(language, {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
      }),
    [language]
  );

  const columns: Array<ColumnDef<AdminGameNoticeView>> = useMemo(() => {
    const formatTime = (value: string) => {
      const parsed = parseTimestamp(value);
      return parsed ? formatter.format(parsed) : "-";
    };

    return [
      {
        accessorKey: "id",
        id: "id",
        header: t("game:notice.id"),
        enableHiding: true,
      },
      {
        accessorKey: "title",
        id: "title",
        header: t("game:notice.title"),
        cell: ({ row }) => {
          const notice = row.original;
          const cat =
            notice.challenge_id != null
              ? getCategory(notice.challenge_category ?? 1)
              : null;
          const CatIcon = cat?.icon;

          return (
            <div className="flex items-center gap-2.5">
              {notice.challenge_id != null && cat && CatIcon ? (
                <div
                  className={cn(
                    "flex size-9 shrink-0 items-center justify-center rounded-md border",
                    "bg-[color-mix(in_oklab,var(--color-cat)_12%,var(--card))]",
                    "border-[color-mix(in_oklab,var(--color-cat)_26%,transparent)]",
                    "text-[color-mix(in_oklab,var(--color-cat)_88%,var(--foreground))]",
                    "dark:bg-[color-mix(in_oklab,var(--color-cat)_22%,var(--card))]",
                    "dark:border-[color-mix(in_oklab,var(--color-cat)_40%,transparent)]",
                    "dark:text-[color-mix(in_oklab,var(--color-cat)_62%,white)]"
                  )}
                  style={{ "--color-cat": cat.color } as CSSProperties}
                >
                  <CatIcon className="size-4" aria-hidden="true" />
                </div>
              ) : (
                <div className="flex size-9 shrink-0 items-center justify-center rounded-md border border-border/70 bg-muted text-muted-foreground">
                  <MegaphoneIcon className="size-4" aria-hidden="true" />
                </div>
              )}
              <div className="min-w-0 flex flex-col gap-1">
                <span className="truncate text-sm font-semibold">
                  {notice.title || "-"}
                </span>
                <div className="flex flex-wrap items-center gap-1.5">
                  <span className="font-mono text-xs text-muted-foreground">
                    #{notice.id}
                  </span>
                  {notice.challenge_id != null &&
                  notice.challenge_title &&
                  cat &&
                  CatIcon ? (
                    <span
                      className={cn(
                        "inline-flex max-w-48 items-center gap-1 rounded-md border px-1.5 py-0.5 text-[11px] font-medium",
                        "bg-[color-mix(in_oklab,var(--color-cat)_10%,var(--card))]",
                        "border-[color-mix(in_oklab,var(--color-cat)_24%,transparent)]",
                        "text-[color-mix(in_oklab,var(--color-cat)_88%,var(--foreground))]",
                        "dark:bg-[color-mix(in_oklab,var(--color-cat)_20%,var(--card))]",
                        "dark:border-[color-mix(in_oklab,var(--color-cat)_38%,transparent)]",
                        "dark:text-[color-mix(in_oklab,var(--color-cat)_62%,white)]"
                      )}
                      style={{ "--color-cat": cat.color } as CSSProperties}
                    >
                      <CatIcon className="size-3 shrink-0" aria-hidden="true" />
                      <span className="truncate">{notice.challenge_title}</span>
                    </span>
                  ) : (
                    <span className="inline-flex items-center gap-1 rounded-md bg-muted px-1.5 py-0.5 text-[11px] text-muted-foreground">
                      <MegaphoneIcon
                        className="size-3 shrink-0"
                        aria-hidden="true"
                      />
                      {t("game:notice.general")}
                    </span>
                  )}
                </div>
              </div>
            </div>
          );
        },
      },
      {
        accessorKey: "content",
        header: t("game:notice.content"),
        cell: ({ row }) => {
          const content = row.original.content;

          if (!content) return "/";

          return content.length > 50 ? (
            <ContentDialog
              title={row.original.title || t("game:notice.content")}
              content={content}
              maxPreviewLength={50}
            />
          ) : (
            content
          );
        },
      },
      {
        id: "author",
        header: t("game:notice.author"),
        cell: ({ row }) => {
          const notice = row.original;

          return (
            <div className="flex flex-col gap-1.5 whitespace-nowrap">
              <div className="flex items-center gap-1.5 text-sm">
                <Clock3Icon
                  className="size-3.5 shrink-0 text-muted-foreground"
                  aria-hidden="true"
                />
                <span className="tabular-nums">
                  {formatTime(notice.updated_at)}
                </span>
                {notice.updated_by_name && (
                  <span className="ml-1 inline-flex min-w-0 items-center gap-1.5 text-xs text-muted-foreground">
                    <Avatar
                      className="size-5 shrink-0"
                      src={
                        notice.updated_by_avatar_hash
                          ? `/api/media?hash=${notice.updated_by_avatar_hash}`
                          : undefined
                      }
                      fallback={notice.updated_by_name.charAt(0)}
                    />
                    <span className="max-w-28 truncate font-medium text-foreground">
                      {notice.updated_by_name}
                    </span>
                  </span>
                )}
              </div>
              <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
                <CalendarPlusIcon
                  className="size-3.5 shrink-0"
                  aria-hidden="true"
                />
                <span className="tabular-nums">
                  {t("game:notice.created_time", {
                    time: formatTime(notice.created_at),
                  })}
                </span>
                {notice.created_by_name && (
                  <span className="ml-1 inline-flex min-w-0 items-center gap-1.5">
                    <Avatar
                      className="size-4 shrink-0"
                      src={
                        notice.created_by_avatar_hash
                          ? `/api/media?hash=${notice.created_by_avatar_hash}`
                          : undefined
                      }
                      fallback={notice.created_by_name.charAt(0)}
                    />
                    <span className="max-w-28 truncate">
                      {notice.created_by_name}
                    </span>
                  </span>
                )}
              </div>
            </div>
          );
        },
      },
      {
        id: "actions",
        header: () => (
          <div className={cn(["justify-self-center"])}>
            {t("game:notice.actions._")}
          </div>
        ),
        cell: ActionsCell,
      },
    ];
  }, [formatter, t]);

  return columns;
}

export { useColumns };

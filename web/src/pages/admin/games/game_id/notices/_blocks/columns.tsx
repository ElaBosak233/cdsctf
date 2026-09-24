import { TrashIcon } from "lucide-react";
import { useContext, useMemo, useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { toast } from "sonner";
import { deleteGameNotice } from "@/api/admin/games/game_id/notices";
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
import type { GameNoticeView } from "@/models/game_notice";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { notifyApiError, parseRouteNumericId } from "@/utils/query";
import { parseTimestamp } from "@/utils/time";
import { Context } from "../../context";

function ActionsCell({ row }: { row: Row<GameNoticeView> }) {
  const { t } = useTranslation();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);
  const sharedStore = useSharedStore();

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

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
        "[&>*]:shrink-0",
      ])}
    >
      <Button
        level={"error"}
        variant={"ghost"}
        size={"sm"}
        square
        icon={<TrashIcon />}
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
            <DialogBody className="px-5">
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
                level={"error"}
                variant={"tonal"}
                size={"sm"}
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
  const { t } = useTranslation();

  const columns: Array<ColumnDef<GameNoticeView>> = useMemo(() => {
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
        cell: ({ row }) => (
          <div className={cn(["min-w-0", "flex", "flex-col", "gap-0.5"])}>
            <span className={cn(["truncate", "text-sm", "font-semibold"])}>
              {row.original.title || "-"}
            </span>
            <span
              className={cn(["font-mono", "text-xs", "text-muted-foreground"])}
            >
              #{row.original.id}
            </span>
          </div>
        ),
      },
      {
        accessorKey: "content",
        header: t("game:notice.content"),
        cell: ({ row }) => {
          const content = row.original.content;

          if (!content) return "-";

          return content.length > 10 ? (
            <ContentDialog title={t("game:notice.content")} content={content} />
          ) : (
            content
          );
        },
      },
      {
        accessorKey: "created_at",
        id: "created_at",
        header: t("game:notice.created_at"),
        cell: ({ row }) => {
          return (
            parseTimestamp(
              row.getValue<string>("created_at")
            )?.toLocaleString() ?? "-"
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
  }, [t]);

  return columns;
}

export { useColumns };

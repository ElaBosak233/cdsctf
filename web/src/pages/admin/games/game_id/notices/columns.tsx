import type { ColumnDef, Row } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import { TrashIcon } from "lucide-react";
import { useMemo, useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { toast } from "sonner";
import { deleteGameNotice } from "@/api/admin/games/game_id/notices";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { ContentDialog } from "@/components/widgets/content-dialog";
import type { GameNotice } from "@/models/game_notice";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

function ActionsCell({ row }: { row: Row<GameNotice> }) {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

  function handleDelete() {
    deleteGameNotice({
      game_id: row.original.game_id,
      id: row.original.id,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(
            t("game.notice.actions.delete.success", {
              title: row.original.title,
            })
          );
          setDeleteDialogOpen(false);
        }
      })
      .finally(() => {
        sharedStore?.setRefresh();
      });
  }

  return (
    <div className={cn(["flex", "items-center", "justify-center", "gap-2"])}>
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
              "flex",
              "flex-col",
              "p-5",
              "min-h-32",
              "w-lg",
              "gap-5",
            ])}
          >
            <div className={cn(["flex", "gap-2", "items-center", "text-sm"])}>
              <TrashIcon className={cn(["size-4", "text-error"])} />
              {t("game.notice.actions.delete._")}
            </div>
            <p className={cn(["text-sm"])}>
              <Trans
                i18nKey="game.notice.actions.delete.message"
                values={{ title: row.original.title }}
                components={{
                  muted: <span className={cn(["text-muted-foreground"])} />,
                }}
              />
            </p>
            <div className={cn(["flex", "justify-end"])}>
              <Button
                level={"error"}
                variant={"tonal"}
                size={"sm"}
                onClick={handleDelete}
              >
                {t("common.actions.confirm")}
              </Button>
            </div>
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function useColumns() {
  const { t } = useTranslation();

  const columns: Array<ColumnDef<GameNotice>> = useMemo(() => {
    return [
      {
        accessorKey: "id",
        id: "id",
        header: "ID",
        cell: function IdCell({ row }) {
          const id = row.original.id;
          return <Badge>{id}</Badge>;
        },
      },
      {
        accessorKey: "title",
        id: "title",
        header: t("game.notice.title"),
        cell: ({ row }) => row.original.title,
      },
      {
        accessorKey: "content",
        header: t("game.notice.content"),
        cell: ({ row }) => {
          const content = row.original.content;

          if (!content) return "-";

          return content.length > 10 ? (
            <ContentDialog title={t("game.notice.content")} content={content} />
          ) : (
            content
          );
        },
      },
      {
        accessorKey: "created_at",
        id: "created_at",
        header: t("game.notice.created_at"),
        cell: ({ row }) => {
          return new Date(
            row.getValue<number>("created_at") * 1000
          ).toLocaleString();
        },
      },
      {
        id: "actions",
        header: () => (
          <div className={cn(["justify-self-center"])}>
            {t("game.notice.actions._")}
          </div>
        ),
        cell: ActionsCell,
      },
    ];
  }, [t]);

  return columns;
}

export { useColumns };

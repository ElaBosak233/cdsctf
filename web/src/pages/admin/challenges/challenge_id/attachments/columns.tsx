import type { ColumnDef, Row } from "@tanstack/react-table";
import { TrashIcon } from "lucide-react";
import prettyBytes from "pretty-bytes";
import { useContext, useMemo, useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { toast } from "sonner";
import { deleteChallengeAttachment } from "@/api/admin/challenges/challenge_id/attachments/filename";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import type { Metadata } from "@/models/media";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";

function ActionsCell({ row }: { row: Row<Metadata> }) {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { challenge } = useContext(Context);

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

  function handleDelete() {
    deleteChallengeAttachment(challenge?.id, row.original.filename)
      .then(() => {
        toast.success(
          t("challenge:attachment.actions.delete.success", {
            filename: row.original.filename,
          })
        );
        setDeleteDialogOpen(false);
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
              "w-lg",
              "rounded-elevated",
              "shadow-lg",
              "overflow-hidden",
              "flex",
              "flex-col",
            ])}
          >
            <div className={cn(["p-5", "flex", "flex-col", "gap-5"])}>
              <div className={cn(["flex", "items-center", "gap-3"])}>
                <div
                  className={cn([
                    "flex items-center justify-center",
                    "size-10 rounded-badge",
                    "bg-error/10 text-error",
                    "shrink-0",
                  ])}
                >
                  <TrashIcon className={cn(["size-5"])} />
                </div>
                <h3 className={cn(["text-base", "font-semibold"])}>
                  {t("challenge:attachment.actions.delete._")}
                </h3>
              </div>
              <p className={cn(["text-sm"])}>
                <Trans
                  i18nKey="challenge:attachment.actions.delete.message"
                  values={{ filename: row.original.filename }}
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
                  {t("common:actions.confirm")}
                </Button>
              </div>
            </div>
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function useColumns() {
  const { t } = useTranslation();

  const columns: Array<ColumnDef<Metadata>> = useMemo(() => {
    return [
      {
        accessorKey: "filename",
        id: "filename",
        header: () => t("challenge:attachment.filename"),
        cell: ({ row }) => row.original.filename,
      },
      {
        accessorKey: "size",
        header: () => t("challenge:attachment.size"),
        cell: ({ row }) => prettyBytes(row.original.size),
      },
      {
        id: "actions",
        header: () => (
          <div className={cn(["justify-self-center"])}>
            {t("challenge:attachment.actions._")}
          </div>
        ),
        cell: ActionsCell,
      },
    ];
  }, [t]);

  return columns;
}

export { useColumns };

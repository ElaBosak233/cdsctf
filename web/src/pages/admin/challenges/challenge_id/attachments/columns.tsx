import { TrashIcon } from "lucide-react";
import prettyBytes from "pretty-bytes";
import { useContext, useMemo, useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { toast } from "sonner";
import { deleteChallengeAttachment } from "@/api/admin/challenges/challenge_id/attachments/filename";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent, DialogHeader } from "@/components/ui/dialog";
import type { ColumnDef, Row } from "@/hooks/use-data-table";
import type { Metadata } from "@/models/media";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";
import { Context } from "../context";

function ActionsCell({ row }: { row: Row<Metadata> }) {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { challenge } = useContext(Context);

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

  async function handleDelete() {
    try {
      await deleteChallengeAttachment(challenge?.id, row.original.filename);
      toast.success(
        t("challenge:attachment.actions.delete.success", {
          filename: row.original.filename,
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
            <div className={cn(["p-5", "flex", "flex-col", "gap-5"])}>
              <DialogHeader
                icon={<TrashIcon />}
                level="error"
                title={t("challenge:attachment.actions.delete._")}
                description={
                  <Trans
                    i18nKey="challenge:attachment.actions.delete.message"
                    values={{ filename: row.original.filename }}
                    components={{
                      muted: <span className={cn(["text-muted-foreground"])} />,
                    }}
                  />
                }
              />
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

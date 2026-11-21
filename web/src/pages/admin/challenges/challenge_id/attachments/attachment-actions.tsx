import { TrashIcon } from "lucide-react";
import { useContext, useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { toast } from "sonner";
import { deleteChallengeAttachment } from "@/api/management/challenges/challenge_id/attachments/filename";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  Dialog,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useSharedStore } from "@/storages/shared";
import { notifyApiError } from "@/utils/query";
import { Context } from "../context";

export function AttachmentActions({ filename }: { filename: string }) {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { challenge } = useContext(Context);

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);
  const [deleting, setDeleting] = useState(false);
  const deleteLabel = t("challenge:attachment.actions.delete._");

  async function handleDelete() {
    setDeleting(true);
    try {
      await deleteChallengeAttachment(challenge?.id, filename);
      toast.success(
        t("challenge:attachment.actions.delete.success", {
          filename,
        })
      );
      setDeleteDialogOpen(false);
      sharedStore.setRefresh();
    } catch (error) {
      await notifyApiError(error);
    } finally {
      setDeleting(false);
    }
  }

  return (
    <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
      <Tooltip>
        <TooltipTrigger
          render={
            <DialogTrigger
              render={
                <Button
                  aria-label={deleteLabel}
                  level="error"
                  variant="ghost"
                  size="sm"
                  square
                  icon={<TrashIcon />}
                />
              }
            />
          }
        />
        <TooltipContent>{deleteLabel}</TooltipContent>
      </Tooltip>
      <DialogContent slotProps={{ title: { children: deleteLabel } }}>
        <Card className="flex w-full max-w-xl flex-col overflow-hidden rounded-elevated shadow-lg">
          <DialogHeader
            className="p-5 pb-0"
            icon={<TrashIcon />}
            level="error"
            title={deleteLabel}
          />
          <DialogBody className="px-5 py-5">
            <p className="text-sm leading-relaxed text-muted-foreground">
              <Trans
                i18nKey="challenge:attachment.actions.delete.message"
                values={{ filename }}
                components={{
                  muted: (
                    <span className="wrap-anywhere font-medium text-foreground" />
                  ),
                }}
              />
            </p>
          </DialogBody>
          <DialogFooter className="p-5 pt-0">
            <Button
              level={"error"}
              variant={"tonal"}
              size={"sm"}
              loading={deleting}
              onClick={handleDelete}
            >
              {t("common:actions.confirm")}
            </Button>
          </DialogFooter>
        </Card>
      </DialogContent>
    </Dialog>
  );
}

import { TrashIcon } from "lucide-react";
import { useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { toast } from "sonner";
import { deleteEmail } from "@/api/admin/users/user_id/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DialogBody, DialogFooter, DialogHeader } from "@/components/ui/dialog";
import { cn } from "@/utils";

interface DeleteEmailDialogProps {
  userId: number;
  email?: string;
  onClose: () => void;
  onSuccess: () => void;
}

export function DeleteEmailDialog(props: DeleteEmailDialogProps) {
  const { userId, email, onClose, onSuccess } = props;
  const { t } = useTranslation();

  const [loading, setLoading] = useState(false);

  async function handleDelete() {
    if (!email) return;
    setLoading(true);
    await deleteEmail({
      user_id: userId,
      email,
    });

    toast.success(t("user:emails.actions.delete.success", { email }));
    onSuccess();
    onClose();

    setLoading(false);
  }

  return (
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
        title={t("user:emails.actions.delete._")}
      />
      <DialogBody className="p-5 pt-4">
        <div
          className={cn([
            "flex",
            "flex-col",
            "gap-1",
            "text-sm",
            "leading-relaxed",
            "text-muted-foreground",
          ])}
        >
          <p className={cn(["font-medium", "text-foreground"])}>
            <Trans
              i18nKey={"user:emails.actions.delete.message"}
              values={{ email }}
              components={{
                muted: <span className={cn(["text-muted-foreground"])} />,
              }}
            />
          </p>
          <p>{t("user:emails.actions.delete.message_brief")}</p>
        </div>
        <DialogFooter>
          <Button
            variant={"solid"}
            level={"error"}
            loading={loading}
            onClick={handleDelete}
            disabled={!email}
          >
            {t("common:actions.confirm")}
          </Button>
        </DialogFooter>
      </DialogBody>
    </Card>
  );
}

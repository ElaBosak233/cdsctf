import { TrashIcon } from "lucide-react";
import { useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { toast } from "sonner";
import { deleteEmail } from "@/api/admin/users/user_id/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
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
            {t("user:emails.actions.delete._")}
          </h3>
        </div>
        <div className={cn(["flex", "flex-col", "gap-1"])}>
          <p className={cn(["text-sm", "font-medium"])}>
            <Trans
              i18nKey={"user:emails.actions.delete.message"}
              values={{ email }}
              components={{
                muted: <span className={cn(["text-muted-foreground"])} />,
              }}
            />
          </p>
          <p className={cn(["text-sm", "text-muted-foreground"])}>
            {t("user:emails.actions.delete.message_brief")}
          </p>
        </div>
        <div className={cn(["flex", "justify-end", "gap-2"])}>
          <Button
            variant={"solid"}
            level={"error"}
            loading={loading}
            onClick={handleDelete}
            disabled={!email}
          >
            {t("common:actions.confirm")}
          </Button>
        </div>
      </div>
    </Card>
  );
}

import { StatusCodes } from "http-status-codes";
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
    const res = await deleteEmail({
      user_id: userId,
      email,
    });

    if (res.code === StatusCodes.OK) {
      toast.success(t("user.emails.actions.delete.success", { email }));
      onSuccess();
      onClose();
    }

    setLoading(false);
  }

  return (
    <Card className={cn(["w-lg", "p-6", "flex", "flex-col", "gap-6"])}>
      <div className={cn(["flex", "items-center", "gap-2", "text-sm"])}>
        <TrashIcon className={cn(["size-4", "text-error"])} />
        {t("user.emails.actions.delete._")}
      </div>
      <div className={cn(["space-y-1"])}>
        <p className={cn(["text-base", "font-medium"])}>
          <Trans
            i18nKey={"user.emails.actions.delete.message"}
            values={{ email }}
            components={{
              muted: <span className={cn(["text-muted-foreground"])} />,
            }}
          />
        </p>
        <p className={cn(["text-muted-foreground", "text-sm"])}>
          {t("user.emails.actions.delete.message_brief")}
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
          {t("common.actions.confirm")}
        </Button>
      </div>
    </Card>
  );
}

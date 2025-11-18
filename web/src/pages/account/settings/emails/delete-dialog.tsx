import { TrashIcon } from "lucide-react";
import { Trans, useTranslation } from "react-i18next";
import { toast } from "sonner";
import { deleteEmail } from "@/api/users/profile/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { cn } from "@/utils";

interface DeleteDialogProps {
  email: string;
  onClose: () => void;
  bump: () => void;
}

function DeleteDialog(props: DeleteDialogProps) {
  const { email, bump, onClose } = props;
  const { t } = useTranslation();

  async function handleDelete() {
    const res = await deleteEmail({
      email: email,
    });

    if (res.code === 200) {
      toast.success(t("user.emails.actions.delete.success", { email }));
      onClose();
      bump();
    }
  }

  return (
    <Card className={cn(["w-lg", "p-5", "flex", "flex-col", "gap-5"])}>
      <div className={cn(["flex", "gap-2", "items-center", "text-sm"])}>
        <TrashIcon className={cn(["size-4", "text-error"])} />
        {t("user.emails.actions.delete._")}
      </div>
      <p className={cn(["text-sm"])}>
        <Trans
          i18nKey={"user.emails.actions.delete.message"}
          values={{ email }}
          components={{
            muted: <span className={cn(["text-muted-foreground"])} />,
          }}
        />
      </p>
      <div className={cn(["flex", "justify-end"])}>
        <Button
          level={"error"}
          variant={"solid"}
          size={"sm"}
          onClick={handleDelete}
        >
          {t("common.actions.confirm")}
        </Button>
      </div>
    </Card>
  );
}

export { DeleteDialog };

import { TrashIcon } from "lucide-react";
import { Trans, useTranslation } from "react-i18next";
import { toast } from "sonner";
import { deleteEmail } from "@/api/users/me/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DialogBody, DialogFooter, DialogHeader } from "@/components/ui/dialog";
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
    await deleteEmail({
      email: email,
    });

    toast.success(t("user:emails.actions.delete.success", { email }));
    onClose();
    bump();
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
        <p className="text-sm leading-relaxed text-muted-foreground">
          <Trans
            i18nKey={"user:emails.actions.delete.message"}
            values={{ email }}
            components={{
              muted: <span className={cn(["text-foreground"])} />,
            }}
          />
        </p>
        <DialogFooter>
          <Button
            level={"error"}
            variant={"solid"}
            size={"sm"}
            onClick={handleDelete}
          >
            {t("common:actions.confirm")}
          </Button>
        </DialogFooter>
      </DialogBody>
    </Card>
  );
}

export { DeleteDialog };

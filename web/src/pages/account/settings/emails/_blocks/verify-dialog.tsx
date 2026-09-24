import { CheckIcon, MailCheckIcon, SendIcon } from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { sendVerifyEmail, verifyEmail } from "@/api/users/me/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DialogHeader } from "@/components/ui/dialog";
import { Field } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { patchAuthenticatedUser, useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";

interface VerifyDialogProps {
  email: string;
  onClose: () => void;
  bump: () => void;
}

function VerifyDialog(props: VerifyDialogProps) {
  const { email, bump, onClose } = props;
  const { t } = useTranslation();

  const user = useAuthStore((state) => state.user);
  const configStore = useConfigStore();

  const [code, setCode] = useState<string>("");

  async function handleSendVerifyEmail() {
    try {
      await sendVerifyEmail({
        email: email,
      });

      toast.success(t("user:emails.actions.send_verify.success", { email }));
    } catch (error) {
      await notifyApiError(error);
    }
  }

  async function handleVerify() {
    try {
      await verifyEmail({
        code: code,
        email: email,
      });

      toast.success(t("user:emails.actions.verify.success", { email }));
      if (user) {
        patchAuthenticatedUser({ verified: true });
      }
      onClose();
      bump();
    } catch (error) {
      await notifyApiError(error);
    }
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
      <div className={cn(["p-5", "flex", "flex-col", "gap-5"])}>
        <DialogHeader
          icon={<MailCheckIcon />}
          title={t("user:emails.actions.verify._")}
        />
        {configStore?.config?.email?.enabled ? (
          <div className={cn(["flex", "gap-2", "items-center"])}>
            <Field size={"sm"} className={cn(["flex-1"])}>
              <TextField
                placeholder={t("user:emails.verify_code_placeholder")}
                value={code}
                onChange={(e) => setCode(e.target.value)}
              />
            </Field>
            <Button
              variant={"solid"}
              icon={<SendIcon />}
              onClick={handleSendVerifyEmail}
            >
              {t("user:emails.actions.send_verify._")}
            </Button>
          </div>
        ) : (
          <div>
            {t("user:emails.actions.verify.disabled", {
              title: configStore?.config?.meta?.title,
            })}
          </div>
        )}
        <Button
          size={"sm"}
          level={"success"}
          variant={"solid"}
          icon={<CheckIcon />}
          onClick={handleVerify}
        >
          {t("common:actions.confirm")}
        </Button>
      </div>
    </Card>
  );
}

export { VerifyDialog };

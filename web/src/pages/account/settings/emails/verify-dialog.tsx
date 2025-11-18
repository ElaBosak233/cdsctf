import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { CheckIcon, MailCheckIcon, SendIcon } from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { sendVerifyEmail, verifyEmail } from "@/api/users/profile/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { parseErrorResponse } from "@/utils/query";

interface VerifyDialogProps {
  email: string;
  onClose: () => void;
  bump: () => void;
}

function VerifyDialog(props: VerifyDialogProps) {
  const { email, bump, onClose } = props;
  const { t } = useTranslation();

  const authStore = useAuthStore();
  const configStore = useConfigStore();

  const [code, setCode] = useState<string>("");

  async function handleSendVerifyEmail() {
    try {
      const res = await sendVerifyEmail({
        email: email,
      });
      if (res.code === StatusCodes.OK) {
        toast.success(t("user.emails.actions.send_verify.success", { email }));
      }
    } catch (error) {
      if (!(error instanceof HTTPError)) return;
      const res = await parseErrorResponse(error);

      if (res.code === StatusCodes.BAD_REQUEST) {
        toast.error(res.msg);
      }
    }
  }

  async function handleVerify() {
    try {
      const res = await verifyEmail({
        code: code,
        email: email,
      });

      if (res.code === StatusCodes.OK) {
        toast.success(t("user.emails.actions.verify.success", { email }));
        authStore.setUser({
          ...authStore.user,
          is_verified: true,
        });
        onClose();
        bump();
      }
    } catch (error) {
      if (!(error instanceof HTTPError)) return;
      const res = await parseErrorResponse(error);

      if (res.code === StatusCodes.BAD_REQUEST) {
        toast.error(res.msg);
      }
    }
  }

  return (
    <Card className={cn(["w-lg", "p-5", "flex", "flex-col", "gap-5"])}>
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <MailCheckIcon className={cn(["size-4"])} />
        {t("user.emails.actions.verify._")}
      </h3>
      {configStore?.config?.email?.is_enabled ? (
        <div className={cn(["flex", "gap-2", "items-center"])}>
          <Field size={"sm"} className={cn(["flex-1"])}>
            <TextField
              placeholder={"I_d0nt_kn0w"}
              value={code}
              onChange={(e) => setCode(e.target.value)}
            />
          </Field>
          <Button
            variant={"solid"}
            icon={<SendIcon />}
            onClick={handleSendVerifyEmail}
          >
            {t("user.emails.actions.send_verify._")}
          </Button>
        </div>
      ) : (
        <div>
          {t("user.emails.actions.verify.disabled", {
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
        {t("common.actions.confirm")}
      </Button>
    </Card>
  );
}

export { VerifyDialog };

import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { CheckIcon, MailCheckIcon, SendIcon } from "lucide-react";
import { useState } from "react";
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
  const authStore = useAuthStore();
  const configStore = useConfigStore();

  const [code, setCode] = useState<string>("");

  async function handleSendVerifyEmail() {
    try {
      const res = await sendVerifyEmail({
        email: email,
      });
      if (res.code === StatusCodes.OK) {
        toast.success("验证码已发送，请查收");
      }
    } catch (error) {
      if (!(error instanceof HTTPError)) return;
      const res = await parseErrorResponse(error);

      if (res.code === StatusCodes.BAD_REQUEST) {
        toast.error("发生错误", {
          description: res.msg,
        });
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
        toast.success("验证成功！");
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
        toast.error("发生错误", {
          description: res.msg,
        });
      }
    }
  }

  return (
    <Card className={cn(["w-128", "p-5", "flex", "flex-col", "gap-5"])}>
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <MailCheckIcon className={cn(["size-4"])} />
        验证邮箱 <span className={cn(["text-muted-foreground"])}>{email}</span>
      </h3>
      {configStore?.config?.email?.is_enabled ? (
        <div className={cn(["flex", "gap-2", "items-center"])}>
          <Field size={"sm"} className={cn(["flex-1"])}>
            <TextField
              placeholder={"验证码"}
              value={code}
              onChange={(e) => setCode(e.target.value)}
            />
          </Field>
          <Button
            variant={"solid"}
            icon={<SendIcon />}
            onClick={handleSendVerifyEmail}
          >
            请求
          </Button>
        </div>
      ) : (
        <div>
          {configStore?.config?.meta?.title}{" "}
          未启用邮件服务，直接确认即可验证邮箱。
        </div>
      )}
      <Button
        size={"sm"}
        variant={"solid"}
        icon={<CheckIcon />}
        onClick={handleVerify}
      >
        提交
      </Button>
    </Card>
  );
}

export { VerifyDialog };

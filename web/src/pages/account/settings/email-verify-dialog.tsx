import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { CheckIcon, MailCheckIcon, SendIcon } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";
import { sendVerifyEmail, verify } from "@/api/users/profile/verify";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { parseErrorResponse } from "@/utils/query";

interface EmailVerifyDialogProps {
  onClose: () => void;
}

function EmailVerifyDialog(props: EmailVerifyDialogProps) {
  const { onClose } = props;
  const authStore = useAuthStore();

  const [code, setCode] = useState<string>("");

  async function handleSendVerifyEmail() {
    try {
      const res = await sendVerifyEmail();
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
      const res = await verify({
        code,
      });

      if (res.code === StatusCodes.OK) {
        toast.success("验证成功！");
        authStore.setUser({
          ...authStore.user,
          is_verified: true,
        });
        onClose();
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
        验证邮箱
      </h3>
      <p>你正在验证邮箱 {authStore?.user?.email}，请接收验证码</p>
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
      <Button variant={"solid"} icon={<CheckIcon />} onClick={handleVerify}>
        提交
      </Button>
    </Card>
  );
}

export { EmailVerifyDialog };

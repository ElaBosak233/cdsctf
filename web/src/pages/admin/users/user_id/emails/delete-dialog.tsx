import { StatusCodes } from "http-status-codes";
import { TrashIcon } from "lucide-react";
import { useState } from "react";
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
  const [loading, setLoading] = useState(false);

  async function handleDelete() {
    if (!email) return;
    setLoading(true);
    const res = await deleteEmail({
      user_id: userId,
      email,
    });

    if (res.code === StatusCodes.OK) {
      toast.success(`邮箱 ${email} 删除成功`);
      onSuccess();
      onClose();
    }

    setLoading(false);
  }

  return (
    <Card className={cn(["w-128", "p-6", "flex", "flex-col", "gap-6"])}>
      <div className={cn(["flex", "items-center", "gap-2", "text-sm"])}>
        <TrashIcon className={cn(["size-4", "text-error"])} />
        删除邮箱
      </div>
      <div className={cn(["space-y-1"])}>
        <p className={cn(["text-base", "font-medium"])}>
          确认删除邮箱{" "}
          <span className={cn(["text-muted-foreground"])}>{email}</span> 吗？
        </p>
        <p className={cn(["text-muted-foreground", "text-sm"])}>
          删除后该邮箱将无法用于登录或邮件通知。
        </p>
      </div>
      <div className={cn(["flex", "justify-end", "gap-2"])}>
        <Button variant={"ghost"} onClick={onClose}>
          取消
        </Button>
        <Button
          variant={"solid"}
          level={"error"}
          loading={loading}
          onClick={handleDelete}
          disabled={!email}
        >
          删除
        </Button>
      </div>
    </Card>
  );
}

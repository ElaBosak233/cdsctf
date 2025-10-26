import { TrashIcon } from "lucide-react";
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

  async function handleDelete() {
    const res = await deleteEmail({
      email: email,
    });

    if (res.code === 200) {
      toast.success(`邮箱 ${email} 删除成功`);
      onClose();
      bump();
    }
  }

  return (
    <Card className={cn(["w-128", "p-5", "flex", "flex-col", "gap-5"])}>
      <div className={cn(["flex", "gap-2", "items-center", "text-sm"])}>
        <TrashIcon className={cn(["size-4"])} />
        删除邮箱 <span className={cn(["text-muted-foreground"])}>{email}</span>
      </div>
      <p className={cn(["text-sm"])}>你确定要删除 {email} 吗？</p>
      <div className={cn(["flex", "justify-end"])}>
        <Button
          level={"error"}
          variant={"tonal"}
          size={"sm"}
          onClick={handleDelete}
        >
          确定
        </Button>
      </div>
    </Card>
  );
}

export { DeleteDialog };

import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { MailIcon, MailPlusIcon, ShieldCheckIcon } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";

import { addEmail } from "@/api/admin/users/user_id/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Switch } from "@/components/ui/switch";
import { TextField } from "@/components/ui/text-field";
import { cn } from "@/utils";

interface CreateEmailDialogProps {
  userId: number;
  onClose: () => void;
  onSuccess: () => void;
}

const formSchema = z.object({
  email: z.email({
    message: "请输入有效的邮箱地址",
  }),
  is_verified: z.boolean(),
});

export function CreateEmailDialog(props: CreateEmailDialogProps) {
  const { userId, onClose, onSuccess } = props;
  const [loading, setLoading] = useState(false);

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      is_verified: false,
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    const res = await addEmail({
      user_id: userId,
      email: values.email,
      is_verified: values.is_verified,
    });

    if (res.code === StatusCodes.OK) {
      toast.success(`邮箱 ${values.email} 添加成功`);
      onSuccess();
      onClose();
      form.reset({
        email: "",
        is_verified: false,
      });
    }

    setLoading(false);
  }

  return (
    <Card className={cn(["w-lg", "p-6", "flex", "flex-col", "gap-6"])}>
      <div className={cn(["flex", "items-center", "gap-2", "text-sm"])}>
        <MailPlusIcon className={cn(["size-4"])} />
        添加邮箱
      </div>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          autoComplete={"off"}
          className={cn(["flex", "flex-col", "gap-6"])}
        >
          <FormField
            control={form.control}
            name={"email"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>邮箱</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <MailIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      placeholder={"ctf@example.com"}
                      value={field.value || ""}
                      onChange={field.onChange}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name={"is_verified"}
            render={({ field }) => (
              <FormItem
                className={cn([
                  "flex",
                  "flex-row",
                  "items-center",
                  "justify-between",
                  "rounded-lg",
                  "border",
                  "p-4",
                ])}
              >
                <div className={cn(["space-y-1"])}>
                  <FormLabel className={cn(["flex", "items-center", "gap-2"])}>
                    <ShieldCheckIcon className={cn(["size-4"])} />
                    标记为已验证
                  </FormLabel>
                  <p className={cn(["text-muted-foreground", "text-sm"])}>
                    立即将邮箱标记为已验证状态。
                  </p>
                </div>
                <FormControl>
                  <Switch
                    checked={field.value}
                    onCheckedChange={(checked) => field.onChange(!!checked)}
                  />
                </FormControl>
              </FormItem>
            )}
          />

          <Button
            type={"submit"}
            variant={"solid"}
            size={"lg"}
            loading={loading}
            level={"primary"}
          >
            确定
          </Button>
        </form>
      </Form>
    </Card>
  );
}

import { zodResolver } from "@hookform/resolvers/zod";
import { MailIcon, MailPlusIcon } from "lucide-react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import z from "zod";
import { addEmail } from "@/api/users/profile/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import { TextField } from "@/components/ui/text-field";
import { cn } from "@/utils";

interface CreateDialogProps {
  onClose: () => void;
  bump: () => void;
}

function CreateDialog(props: CreateDialogProps) {
  const { onClose, bump } = props;

  const formSchema = z.object({
    email: z.email("邮箱不合法"),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });
  async function onSubmit(values: z.infer<typeof formSchema>) {
    const res = await addEmail({
      email: values.email,
    });

    if (res.code === 200) {
      toast.success(`邮箱 ${values.email} 添加成功`);
      onClose();
      bump();
    }
  }

  return (
    <Card className={cn(["w-lg", "p-5", "flex", "flex-col", "gap-5"])}>
      <div className={cn(["flex", "gap-2", "items-center", "text-sm"])}>
        <MailPlusIcon className={cn(["size-4"])} />
        添加邮箱
      </div>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          autoComplete={"off"}
          className={cn(["flex", "flex-col", "h-full", "gap-8"])}
        >
          <FormField
            control={form.control}
            name={"email"}
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <MailIcon />
                    </FieldIcon>
                    <TextField placeholder={"ctf@example.com"} {...field} />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button
            level={"success"}
            variant={"solid"}
            size={"sm"}
            type={"submit"}
          >
            确定
          </Button>
        </form>
      </Form>
    </Card>
  );
}

export { CreateDialog };

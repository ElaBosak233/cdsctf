import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  CheckIcon,
  MailIcon,
  MailPlusIcon,
  ShieldCheckIcon,
} from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
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

export function CreateEmailDialog(props: CreateEmailDialogProps) {
  const { userId, onClose, onSuccess } = props;
  const { t } = useTranslation();

  const [loading, setLoading] = useState(false);

  const formSchema = z.object({
    email: z.email({
      message: t("user.emails.form.email.message"),
    }),
    is_verified: z.boolean(),
  });

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
      toast.success(
        t("user.emails.actions.create.success", { email: values.email })
      );
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
        {t("user.emails.actions.create._")}
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
                <FormLabel>{t("user.emails.form.email._")}</FormLabel>
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
                    {t("user.emails.form.is_verified._")}
                  </FormLabel>
                  <p className={cn(["text-muted-foreground", "text-sm"])}>
                    {t("user.emails.form.is_verified.message")}
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
            icon={<CheckIcon />}
            size={"lg"}
            loading={loading}
            level={"success"}
          >
            {t("common.actions.confirm")}
          </Button>
        </form>
      </Form>
    </Card>
  );
}

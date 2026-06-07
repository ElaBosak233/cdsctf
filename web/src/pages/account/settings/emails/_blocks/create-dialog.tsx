import { zodResolver } from "@hookform/resolvers/zod";
import { CheckIcon, MailIcon, MailPlusIcon } from "lucide-react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import z from "zod";
import { addEmail } from "@/api/users/me/emails";
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
  const { t } = useTranslation();

  const formSchema = z.object({
    email: z.email(t("user:emails.form.email.message")),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });
  async function onSubmit(values: z.infer<typeof formSchema>) {
    await addEmail({
      email: values.email,
    });

    toast.success(
      t("user:emails.actions.create.success", { email: values.email })
    );
    onClose();
    bump();
  }

  return (
    <Card
      className={cn([
        "w-lg",
        "rounded-elevated",
        "shadow-lg",
        "overflow-hidden",
        "flex",
        "flex-col",
      ])}
    >
      <div className={cn(["p-5", "flex", "flex-col", "gap-5"])}>
        <div className={cn(["flex", "items-center", "gap-3"])}>
          <div
            className={cn([
              "flex items-center justify-center",
              "size-10 rounded-badge",
              "bg-primary/10",
              "shrink-0",
            ])}
          >
            <MailPlusIcon className={cn(["size-5"])} />
          </div>
          <h3 className={cn(["text-base", "font-semibold"])}>
            {t("user:emails.actions.create._")}
          </h3>
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
                      <TextField
                        placeholder={t("user:emails.form.email.placeholder")}
                        {...field}
                      />
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button
              icon={<CheckIcon />}
              level={"success"}
              variant={"solid"}
              size={"sm"}
              type={"submit"}
            >
              {t("common:actions.confirm")}
            </Button>
          </form>
        </Form>
      </div>
    </Card>
  );
}

export { CreateDialog };

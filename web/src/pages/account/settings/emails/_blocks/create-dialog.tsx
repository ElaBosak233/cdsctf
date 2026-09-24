import { zodResolver } from "@hookform/resolvers/zod";
import { CheckIcon, MailIcon, MailPlusIcon } from "lucide-react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import z from "zod";
import { addEmail } from "@/api/users/me/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DialogBody, DialogFooter, DialogHeader } from "@/components/ui/dialog";
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
        "w-full",
        "max-w-xl",
        "rounded-elevated",
        "shadow-lg",
        "overflow-hidden",
        "flex",
        "flex-col",
      ])}
    >
      <DialogHeader
        className="p-5 pb-0"
        icon={<MailPlusIcon />}
        title={t("user:emails.actions.create._")}
      />
      <DialogBody className="p-5 pt-4">
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
            <DialogFooter>
              <Button
                icon={<CheckIcon />}
                level={"success"}
                variant={"solid"}
                size={"sm"}
                type={"submit"}
              >
                {t("common:actions.confirm")}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogBody>
    </Card>
  );
}

export { CreateDialog };

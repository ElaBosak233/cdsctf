import { zodResolver } from "@hookform/resolvers/zod";
import {
  CheckIcon,
  KeyIcon,
  MailIcon,
  UserRoundCheckIcon,
  UserRoundIcon,
  UserRoundPlusIcon,
} from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { createUser } from "@/api/management/users";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DialogBody, DialogFooter, DialogHeader } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { TextField } from "@/components/ui/text-field";
import { BASIC_PERMISSION, PERMISSIONS } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";
import { PermissionChecklist } from "./permission-checklist";

interface CreateUserDialogProps {
  onClose: () => void;
}

function CreateUserDialog(props: CreateUserDialogProps) {
  const { onClose } = props;
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z.object({
    username: z
      .string()
      .min(3, { message: t("user:form.username.messages.min") }),
    name: z.string().min(2, { message: t("user:form.name.messages._") }),
    email: z.email({ message: t("user:form.email.message") }),
    password: z
      .string()
      .min(6, { message: t("user:form.password.messages.min") }),
    permissions: z.array(z.enum(PERMISSIONS)),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      permissions: [BASIC_PERMISSION],
      username: "",
      name: "",
      email: "",
      password: "",
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    try {
      await createUser({
        ...values,
      });
      toast.success(
        t("user:actions.create.success", { username: values.username })
      );
      onClose();
    } catch (error) {
      await notifyApiError(error);
    } finally {
      sharedStore.setRefresh();
      setLoading(false);
    }
  }

  return (
    <Card
      className={cn([
        "w-full",
        "max-w-xl",
        "min-h-64",
        "rounded-elevated",
        "shadow-lg",
        "overflow-hidden",
        "flex",
        "flex-col",
      ])}
    >
      <DialogHeader
        className="p-5 pb-0"
        icon={<UserRoundPlusIcon />}
        title={t("user:actions.create._")}
      />
      <DialogBody className="p-5 pt-4">
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            autoComplete={"off"}
            className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
          >
            <FormField
              control={form.control}
              name={"username"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("user:form.username._")}</FormLabel>
                  <FormControl>
                    <Field size={"sm"}>
                      <FieldIcon>
                        <UserRoundIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={t("user:form.username.placeholder")}
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
              name={"name"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("user:form.name._")}</FormLabel>
                  <FormControl>
                    <Field size={"sm"}>
                      <FieldIcon>
                        <UserRoundCheckIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={t("user:form.name.placeholder")}
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
              name={"email"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("user:form.email._")}</FormLabel>
                  <FormControl>
                    <Field size={"sm"}>
                      <FieldIcon>
                        <MailIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        type="email"
                        placeholder={t("user:form.email.placeholder")}
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
              name={"password"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("user:form.password._")}</FormLabel>
                  <FormControl>
                    <Field size={"sm"}>
                      <FieldIcon>
                        <KeyIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        type="password"
                        placeholder={t("user:form.password.placeholder")}
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
              name="permissions"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("user:form.permissions._")}</FormLabel>
                  <FormControl>
                    <PermissionChecklist
                      value={field.value}
                      onChange={field.onChange}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <DialogFooter>
              <Button
                type={"submit"}
                variant={"solid"}
                icon={<CheckIcon />}
                level={"success"}
                loading={loading}
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

export { CreateUserDialog };

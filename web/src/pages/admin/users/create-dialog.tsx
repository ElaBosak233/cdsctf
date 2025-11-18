import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  CheckIcon,
  KeyIcon,
  MailIcon,
  ShieldIcon,
  UserRoundCheckIcon,
  UserRoundIcon,
  UserRoundPlusIcon,
} from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { createUser } from "@/api/admin/users";
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
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { Group } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

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
      .min(3, { message: t("user.form.username.messages.min") }),
    name: z.string().min(2, { message: t("user.form.name.messages._") }),
    email: z.email({ message: t("user.form.email.message") }),
    password: z
      .string()
      .min(6, { message: t("user.form.password.messages.min") }),
    group: z.number({
      message: t("user.form.group.message"),
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      group: Group.User,
      username: "",
      name: "",
      email: "",
      password: "",
    },
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    createUser({
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(
            t("user.actions.create.success", { username: values.username })
          );
          onClose();
        }

        if (res.code === StatusCodes.CONFLICT) {
          toast.error(t("user.actions.create.errors.conflict"));
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  const groupOptions = [
    { id: Group.User, name: t("user.group.user"), icon: UserRoundCheckIcon },
    { id: Group.Admin, name: t("user.group.admin"), icon: ShieldIcon },
  ];

  return (
    <Card
      className={cn(["w-lg", "min-h-64", "p-5", "flex", "flex-col", "gap-5"])}
    >
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <UserRoundPlusIcon className={cn(["size-4"])} />
        {t("user.actions.create._")}
      </h3>
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
                <FormLabel>{t("user.form.username._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <UserRoundIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      placeholder={"Username"}
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
                <FormLabel>{t("user.form.name._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <UserRoundCheckIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      placeholder={"Name"}
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
                <FormLabel>{t("user.form.email._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <MailIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      type="email"
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
            name={"password"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("user.form.password._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <KeyIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      type="password"
                      placeholder={"P4ssw0rd!"}
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
            name={"group"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("user.form.group._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <UserRoundCheckIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      options={groupOptions.map((group) => {
                        const Icon = group.icon;
                        return {
                          value: String(group.id),
                          content: (
                            <div
                              className={cn(["flex", "gap-2", "items-center"])}
                            >
                              <Icon className="size-4" />
                              {group.name}
                            </div>
                          ),
                        };
                      })}
                      onValueChange={(value) => {
                        field.onChange(Number(value));
                      }}
                      value={String(field.value)}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button
            type={"submit"}
            variant={"solid"}
            icon={<CheckIcon />}
            level={"success"}
            loading={loading}
          >
            {t("common.actions.confirm")}
          </Button>
        </form>
      </Form>
    </Card>
  );
}

export { CreateUserDialog };

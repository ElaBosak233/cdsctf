import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  SaveIcon,
  ShieldIcon,
  UserRoundCheckIcon,
  UserRoundIcon,
  UserRoundXIcon,
} from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import { updateUser } from "@/api/admin/users/user_id";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Editor } from "@/components/ui/editor";
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
import { Context } from "./context";

export default function Index() {
  const { t } = useTranslation();

  const { user } = useContext(Context);
  const { user_id } = useParams<{ user_id: string }>();

  const sharedStore = useSharedStore();
  const [loading, setLoading] = useState<boolean>(false);

  const groupOptions = [
    {
      id: Group.Guest.toString(),
      name: t("user.group.guest"),
      icon: UserRoundIcon,
    },
    {
      id: Group.Banned.toString(),
      name: t("user.group.banned"),
      icon: UserRoundXIcon,
    },
    {
      id: Group.User.toString(),
      name: t("user.group.user"),
      icon: UserRoundCheckIcon,
    },
    {
      id: Group.Admin.toString(),
      name: t("user.group.admin"),
      icon: ShieldIcon,
    },
  ];

  const formSchema = z.object({
    username: z.string({}),
    name: z.string({
      message: t("user.form.name.messages._"),
    }),
    group: z.number({
      message: t("user.form.group.message"),
    }),
    description: z.string().nullish(),
    is_verified: z.boolean(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: user,
  });

  useEffect(() => {
    form.reset(user, {
      keepDefaultValues: false,
    });
  }, [user, form]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    if (!user) return;

    setLoading(true);
    updateUser({
      id: user.id!,
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(
            t("user.actions.update.success", { username: res.data?.username })
          );
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  return (
    <div className={cn(["flex", "flex-col", "gap-6", "flex-1"])}>
      <div className={cn(["flex", "flex-col", "items-center", "gap-4"])}>
        <Avatar
          className={cn(["h-30", "w-30"])}
          src={user?.has_avatar && `/api/users/${user_id}/avatar`}
          fallback={user?.username?.charAt(0)}
        />
      </div>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          autoComplete={"off"}
          className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
        >
          <div
            className={cn(["grid", "grid-cols-1", "md:grid-cols-3", "gap-4"])}
          >
            <FormField
              control={form.control}
              name={"username"}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>{t("user.form.username._")}</FormLabel>
                  <FormControl>
                    <Field disabled>
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
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>{t("user.form.name._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <UserRoundIcon />
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
              name={"group"}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>{t("user.form.group._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <ShieldIcon />
                      </FieldIcon>
                      <Select
                        {...field}
                        options={groupOptions.map((group) => ({
                          value: group.id,
                          content: (
                            <div
                              className={cn(["flex", "gap-2", "items-center"])}
                            >
                              <group.icon className="size-4" />
                              {group.name}
                            </div>
                          ),
                        }))}
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
          </div>

          <FormField
            control={form.control}
            name={"description"}
            render={({ field }) => (
              <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
                <FormLabel>{t("user.form.description._")}</FormLabel>
                <FormControl>
                  <Editor
                    {...field}
                    value={field.value ?? ""}
                    className={cn(["h-full", "min-h-64"])}
                    lang="markdown"
                    tabSize={2}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <Button
            size={"lg"}
            variant={"solid"}
            level={"primary"}
            type={"submit"}
            icon={<SaveIcon />}
            loading={loading}
            className={cn(["w-full"])}
          >
            {t("common.actions.save")}
          </Button>
        </form>
      </Form>
    </div>
  );
}

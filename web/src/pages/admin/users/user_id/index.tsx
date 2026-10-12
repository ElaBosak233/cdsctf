import { zodResolver } from "@hookform/resolvers/zod";
import { InfoIcon, SaveIcon, UserRoundIcon } from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import { updateUser } from "@/api/management/users/user_id";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { MarkdownEditor } from "@/components/ui/markdown-editor";
import { Separator } from "@/components/ui/separator";
import { TextField } from "@/components/ui/text-field";
import { PERMISSIONS } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";
import { PermissionChecklist } from "../_blocks/permission-checklist";
import { Context } from "./context";

export default function Index() {
  const { t } = useTranslation();

  const { user } = useContext(Context);
  useParams<{ user_id: string }>();

  const sharedStore = useSharedStore();
  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z.object({
    username: z.string({}),
    name: z.string({
      message: t("user:form.name.messages._"),
    }),
    permissions: z.array(z.enum(PERMISSIONS)),
    description: z.string().nullish(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      username: user?.username ?? "",
      name: user?.name ?? "",
      permissions: user?.permissions ?? [],
      description: user?.description ?? null,
    },
  });

  useEffect(() => {
    if (!user) return;
    form.reset(
      {
        username: user.username,
        name: user.name,
        permissions: user.permissions,
        description: user.description,
      },
      {
        keepDefaultValues: false,
      }
    );
  }, [user, form]);

  async function onSubmit(values: z.infer<typeof formSchema>) {
    if (!user) return;

    setLoading(true);
    try {
      const res = await updateUser({
        id: user.id!,
        ...values,
      });
      toast.success(
        t("user:actions.update.success", { username: res.user?.username })
      );
    } catch (error) {
      await notifyApiError(error);
    } finally {
      sharedStore.setRefresh();
      setLoading(false);
    }
  }

  return (
    <div className={cn(["flex", "flex-col", "gap-5", "flex-1"])}>
      <div className={cn(["flex", "items-center", "gap-3"])}>
        <div
          className={cn([
            "flex items-center justify-center",
            "size-10 rounded-badge",
            "bg-primary/10",
            "shrink-0",
          ])}
        >
          <InfoIcon className={cn(["size-5"])} />
        </div>
        <h2 className={cn(["text-base", "font-semibold"])}>
          {t("user:edit.info")}
        </h2>
      </div>
      <Separator />
      <div className={cn(["flex", "flex-col", "items-center", "gap-4"])}>
        <Avatar
          className={cn(["h-30", "w-30"])}
          src={user?.avatar_hash && `/api/media?hash=${user?.avatar_hash}`}
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
            className={cn(["grid", "grid-cols-1", "md:grid-cols-2", "gap-4"])}
          >
            <FormField
              control={form.control}
              name={"username"}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>{t("user:form.username._")}</FormLabel>
                  <FormControl>
                    <Field disabled>
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
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>{t("user:form.name._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <UserRoundIcon />
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
          </div>

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

          <FormField
            control={form.control}
            name={"description"}
            render={({ field }) => (
              <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
                <FormLabel>{t("user:form.description._")}</FormLabel>
                <FormControl>
                  <MarkdownEditor
                    {...field}
                    value={field.value ?? ""}
                    className={cn(["h-full", "min-h-64"])}
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
            {t("common:actions.save")}
          </Button>
        </form>
      </Form>
    </div>
  );
}

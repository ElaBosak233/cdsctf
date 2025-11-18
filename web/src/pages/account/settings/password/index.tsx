import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { LockIcon, LockOpenIcon, SaveIcon } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { updateUserProfilePassword } from "@/api/users/profile";
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
import { TextField } from "@/components/ui/text-field";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function Index() {
  const { t } = useTranslation();

  const configStore = useConfigStore();

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z
    .object({
      old_password: z.string({
        message: t("user.change_password.form.old_password.message"),
      }),
      new_password: z
        .string({
          message: t("user.change_password.form.new_password.messages._"),
        })
        .min(6, t("user.change_password.form.new_password.messages.min")),
      confirm_password: z.string({
        message: t("user.change_password.form.confirm_password.messages._"),
      }),
    })
    .refine((data) => data.new_password === data.confirm_password, {
      message: t("user.change_password.form.confirm_password.messages.match"),
      path: ["confirm_password"],
    });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      old_password: "",
      new_password: "",
      confirm_password: "",
    },
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    updateUserProfilePassword({
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(t("user.change_password.actions.self_update.success"));
          form.reset();
        }

        if (res.code === StatusCodes.BAD_REQUEST) {
          toast.error(res.msg);
        }
      })
      .finally(() => {
        setLoading(false);
      });
  }

  return (
    <>
      <title>{`${t("user.settings.password")} - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col",
          "flex-1",
          "p-10",
          "xl:mx-50",
          "lg:mx-30",
        ])}
      >
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            autoComplete={"off"}
            className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
          >
            <FormField
              control={form.control}
              name={"old_password"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>
                    {t("user.change_password.form.old_password._")}
                  </FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <LockOpenIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        type={"password"}
                        placeholder={"Old P4ssw0rd"}
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
              name={"new_password"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>
                    {t("user.change_password.form.new_password._")}
                  </FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <LockIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        type={"password"}
                        placeholder={"New P4ssw0rd"}
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
              name={"confirm_password"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>
                    {t("user.change_password.form.confirm_password._")}
                  </FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <LockIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        type={"password"}
                        placeholder={"Confirm New P4ssw0rd"}
                        value={field.value || ""}
                        onChange={field.onChange}
                      />
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className={cn(["flex-1"])} />

            <Button
              size={"lg"}
              type={"submit"}
              level={"primary"}
              variant={"solid"}
              icon={<SaveIcon />}
              loading={loading}
            >
              {t("common.actions.save")}
            </Button>
          </form>
        </Form>
      </div>
    </>
  );
}

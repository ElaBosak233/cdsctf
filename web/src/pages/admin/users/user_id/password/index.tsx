import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { LockIcon, SaveIcon } from "lucide-react";
import { useContext, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { updateUser } from "@/api/admin/users/user_id";
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
import { cn } from "@/utils";
import { Context } from "../context";

export default function Index() {
  const { t } = useTranslation();

  const { user } = useContext(Context);
  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z
    .object({
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
      new_password: "",
      confirm_password: "",
    },
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    if (!user) return;

    setLoading(true);
    updateUser({
      id: user.id!,
      password: values.new_password,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(
            t("user.change_password.actions.update.success", {
              username: user.username,
            })
          );
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
    <div className={cn(["flex", "flex-col", "flex-1"])}>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          autoComplete={"off"}
          className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
        >
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
            variant={"solid"}
            icon={<SaveIcon />}
            loading={loading}
          >
            {t("common.actions.save")}
          </Button>
        </form>
      </Form>
    </div>
  );
}

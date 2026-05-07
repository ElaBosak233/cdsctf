import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import {
  CheckCheckIcon,
  LockIcon,
  TriangleAlertIcon,
  UserRoundIcon,
} from "lucide-react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import { deleteUserProfile } from "@/api/users/me";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Separator } from "@/components/ui/separator";
import { TextField } from "@/components/ui/text-field";
import { Typography } from "@/components/ui/typography";
import { Captcha } from "@/components/widgets/captcha";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { formatApiMsg, parseErrorResponse } from "@/utils/query";

export default function Index() {
  const { t } = useTranslation();

  const configStore = useConfigStore();
  const authStore = useAuthStore();
  const navigate = useNavigate();

  const formSchema = z
    .object({
      username: z.string({
        message: t("user:delete_account.form.username.messages._"),
      }),
      password: z.string({
        message: t("user:form.password.messages._"),
      }),
      captcha: z
        .object({
          id: z.string(),
          content: z.string(),
        })
        .nullish(),
    })
    .refine((data) => data.username === authStore?.user?.username, {
      message: t("user:delete_account.form.username.messages.match"),
      path: ["username"],
    });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    try {
      await deleteUserProfile({
        ...values,
      });
      toast.success(t("user:delete_account.actions.delete.success"));
      authStore?.clear();
      navigate("/");
    } catch (error) {
      if (!(error instanceof HTTPError)) throw error;
      const body = await parseErrorResponse(error);
      if (error.response.status === StatusCodes.BAD_REQUEST) {
        toast.error(formatApiMsg(body.msg));
      }
    }
  }

  return (
    <>
      <title>{`${t("user:delete_account._")} - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col",
          "flex-1",
          "p-10",
          "xl:mx-50",
          "lg:mx-30",
          "gap-8",
        ])}
      >
        <Typography className={cn(["space-y-5"])}>
          <div
            className={cn([
              "flex",
              "justify-center",
              "items-center",
              "text-warning",
              "gap-3",
            ])}
          >
            <TriangleAlertIcon className={cn(["size-12"])} />
            <span className={cn(["text-xl", "font-semibold"])}>
              {t("user:delete_account.final_warning")}
            </span>
          </div>
          <Separator />
          <p className={cn(["font-bold"])}>
            {t("user:delete_account.warnings.intro")}
          </p>
          <ul>
            <li>{t("user:delete_account.warnings.username")}</li>
            <li>{t("user:delete_account.warnings.email")}</li>
            <li>{t("user:delete_account.warnings.data")}</li>
          </ul>
          <p className={cn(["font-bold"])}>
            {t("user:delete_account.warnings.irreversible")}
          </p>
          <p className={cn(["text-error"])}>
            {t("user:delete_account.warnings.confirm")}
          </p>
        </Typography>
        <div className={cn(["flex-1"])} />
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            autoComplete={"off"}
            className={cn(["flex", "flex-col", "gap-5"])}
          >
            <div className={cn(["flex", "gap-5"])}>
              <FormField
                name={"username"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>
                      {t("user:delete_account.form.username._")}
                    </FormLabel>
                    <Field>
                      <FieldIcon>
                        <UserRoundIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={authStore?.user?.username}
                      />
                    </Field>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                name={"password"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>
                      {t("user:delete_account.form.password._")}
                    </FormLabel>
                    <Field>
                      <FieldIcon>
                        <LockIcon />
                      </FieldIcon>
                      <TextField {...field} type={"password"} />
                    </Field>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
            {configStore?.config?.captcha?.provider !== "none" && (
              <FormField
                name={"captcha"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>
                      {t("user:delete_account.form.captcha._")}
                    </FormLabel>
                    <Captcha onChange={field.onChange} />
                  </FormItem>
                )}
              />
            )}
            <Button
              variant={"solid"}
              size={"lg"}
              level={"error"}
              icon={<CheckCheckIcon />}
              type={"submit"}
            >
              {t("common:actions.confirm")}
            </Button>
          </form>
        </Form>
      </div>
    </>
  );
}

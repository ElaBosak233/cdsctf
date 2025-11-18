import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { CheckIcon, LockIcon, MailIcon, SendIcon } from "lucide-react";
import { useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { z } from "zod";

import { forget, sendForgetEmail } from "@/api/users/forget";
import { Button } from "@/components/ui/button";
import { Field, FieldButton, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { TextField } from "@/components/ui/text-field";
import { Captcha, type CaptchaRef } from "@/components/widgets/captcha";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

function ForgetForm() {
  const configStore = useConfigStore();
  const authStore = useAuthStore();
  const navigate = useNavigate();
  const { t } = useTranslation();

  const captchaRef = useRef<CaptchaRef>(null);

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z.object({
    email: z
      .string({
        message: t("account.forget.form.email.message"),
      })
      .email(),
    code: z.string({
      message: t("account.forget.form.code.message"),
    }),
    password: z.string({
      message: t("account.forget.form.password.message"),
    }),
    captcha: z
      .object({
        id: z.string(),
        content: z.string(),
      })
      .nullish(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    forget({
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          authStore.setUser(res.data);
          toast.success(t("account.forget.toast.success._"), {
            description: t("account.forget.toast.success.desc"),
          });
          navigate("/account/login");
        }

        if (res.code === StatusCodes.BAD_REQUEST) {
          toast.error(t("common.errors.default"), {
            description: res.msg,
          });
        }

        captchaRef.current?.refresh();
      })
      .finally(() => {
        setLoading(false);
      });
  }

  function handleSendForgetEmail() {
    sendForgetEmail({
      email: form.getValues().email,
    }).then((res) => {
      if (res.code === StatusCodes.OK) {
        toast.success(t("account.forget.toast.code_sent"));
      }

      if (res.code === StatusCodes.BAD_REQUEST) {
        toast.error(t("common.errors.default"), {
          description: res.msg,
        });
      }

      if (res.code === StatusCodes.NOT_FOUND) {
        toast.error(t("account.forget.toast.not_found"));
      }
    });
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        autoComplete={"off"}
        className={cn(["flex", "flex-col", "h-full", "gap-8"])}
      >
        <div className={cn("space-y-3", "flex-1")}>
          <FormField
            control={form.control}
            name={"email"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("account.forget.form.email._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <MailIcon />
                    </FieldIcon>
                    <TextField
                      placeholder={t("account.forget.form.email._")}
                      {...field}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <div className={cn(["flex", "gap-2", "items-center"])}>
            <FormField
              control={form.control}
              name={"code"}
              render={({ field }) => (
                <FormItem className={cn(["flex-1"])}>
                  <FormLabel>{t("account.forget.form.code._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <MailIcon />
                      </FieldIcon>
                      <TextField
                        placeholder={t("account.forget.form.code._")}
                        {...field}
                      />
                      <FieldButton
                        icon={<SendIcon />}
                        onClick={handleSendForgetEmail}
                        className={cn(["aspect-auto"])}
                        disabled={!form.watch("email")?.trim()}
                      >
                        {t("account.forget.form.code.request")}
                      </FieldButton>
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
          <FormField
            control={form.control}
            name={"password"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("account.forget.form.password._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <LockIcon />
                    </FieldIcon>
                    <TextField
                      placeholder={t("account.forget.form.password._")}
                      type={"password"}
                      {...field}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          {configStore?.config?.captcha?.provider !== "none" && (
            <FormField
              name={"captcha"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("account.forget.form.captcha._")}</FormLabel>
                  <Captcha
                    ref={captchaRef}
                    onChange={field.onChange}
                  />
                </FormItem>
              )}
            />
          )}
        </div>
        <Button
          variant={"solid"}
          level={"info"}
          type={"submit"}
          size={"lg"}
          className={cn(["w-full"])}
          icon={<CheckIcon />}
          loading={loading}
        >
          {t("account.forget.form.submit")}
        </Button>
      </form>
    </Form>
  );
}

export { ForgetForm };
